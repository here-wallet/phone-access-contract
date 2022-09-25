use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::json_types::ValidAccountId;
use near_sdk::json_types::U128;
use near_sdk::{assert_one_yocto, ext_contract, Balance, Promise};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use serde::Serialize;

pub static GAS: u64 = 10_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
pub struct NearTrustTransaction {
    from_account_id: AccountId,
    amount: Balance,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
pub struct NftTrustTransaction {
    from_account_id: AccountId,
    nft_contract_id: AccountId,
    nft_token_id: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
pub struct FtTrustTransaction {
    from_account_id: AccountId,
    ft_contract_id: AccountId,
    ft_amount: Balance,
}

#[ext_contract(nft_contract)]
pub trait NftContract {
    fn nft_transfer(&self, receiver_id: String, token_id: String);
}

#[ext_contract(ft_contract)]
pub trait FtContract {
    fn ft_transfer(&self, receiver_id: String, amount: U128);
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    phone_to_user: UnorderedMap<String, AccountId>,
    near_trusts: UnorderedMap<String, Vector<NearTrustTransaction>>,
    ft_trusts: UnorderedMap<String, Vector<FtTrustTransaction>>,
    nft_trusts: UnorderedMap<String, Vector<NftTrustTransaction>>,
    owner_id: String,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner_id: owner_id,
            phone_to_user: UnorderedMap::new(b"p".to_vec()),
            near_trusts: UnorderedMap::new(b"n".to_vec()),
            ft_trusts: UnorderedMap::new(b"ft".to_vec()),
            nft_trusts: UnorderedMap::new(b"nft".to_vec()),
        }
    }

    #[payable]
    pub fn allocate_phone(&mut self, phone: String, account_id: ValidAccountId) {
        self.assert_owner();
        assert_one_yocto();
        assert!(
            self.phone_to_user.get(&phone).is_none(),
            "the phone is already allocated"
        );
        self.phone_to_user.insert(&phone, &account_id.to_string());

        let near_trusts = self.near_trusts.get(&phone);
        if near_trusts.is_some() {
            let mut balance = 0;
            for nt in near_trusts.unwrap().iter() {
                env::log(format!("Add {} from {}", nt.amount, nt.from_account_id).as_bytes());
                balance += nt.amount;
            }
            Promise::new(account_id.clone().to_string()).transfer(balance);
            self.near_trusts.remove(&phone);
        }

        let nft_trusts = self.nft_trusts.get(&phone);
        if nft_trusts.is_some() {
            for nft in nft_trusts.unwrap().iter() {
                env::log(format!("Send NFT {}", nft.nft_token_id).as_bytes());
                nft_contract::nft_transfer(
                    account_id.clone().to_string(),
                    nft.nft_token_id,
                    &nft.nft_contract_id,
                    1,
                    GAS,
                );
            }
            self.nft_trusts.remove(&phone);
        }

        let ft_trusts = self.ft_trusts.get(&phone);
        if ft_trusts.is_some() {
            for ft in ft_trusts.unwrap().iter() {
                env::log(format!("Send FT {}", ft.ft_contract_id).as_bytes());
                ft_contract::ft_transfer(
                    account_id.clone().to_string(),
                    U128(ft.ft_amount),
                    &ft.ft_contract_id,
                    1,
                    GAS,
                );
            }
            self.ft_trusts.remove(&phone);
        }
    }

    #[payable]
    pub fn delete_phone(&mut self, phone: String) {
        assert_one_yocto();
        let account_id = self.phone_to_user.get(&phone).unwrap();
        assert!(
            account_id == env::predecessor_account_id(),
            "Not access to this phone"
        );
        self.phone_to_user.remove(&phone);
    }

    #[payable]
    pub fn send_near_to_phone(&mut self, phone: String) {
        let account_id = self.phone_to_user.get(&phone);

        if account_id.is_some() {
            Promise::new(account_id.unwrap()).transfer(env::attached_deposit());
        } else {
            let mut near_trusts = self
                .near_trusts
                .get(&phone)
                .unwrap_or_else(|| Vector::new([phone.as_bytes(), b"n"].concat()));

            near_trusts.push(&NearTrustTransaction {
                from_account_id: env::predecessor_account_id(),
                amount: env::attached_deposit(),
            });
            self.near_trusts.insert(&phone, &near_trusts);
        }
    }

    pub fn change_owner(&mut self, new_owner_id: ValidAccountId) {
        self.assert_owner();
        self.owner_id = new_owner_id.into();
        env::log("Owner changed".as_bytes());
    }

    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "ERR_NOT_ALLOWED"
        );
    }

    pub fn nft_on_transfer(
        &mut self,
        sender_id: String,
        previous_owner_id: String,
        token_id: String,
        msg: String,
    ) -> bool {
        let account_id = self.phone_to_user.get(&msg);

        if account_id.is_some() {
            nft_contract::nft_transfer(
                account_id.clone().unwrap(),
                token_id,
                &env::predecessor_account_id(),
                1,
                GAS,
            );
        } else {
            let mut nft_trusts = self
                .nft_trusts
                .get(&msg)
                .unwrap_or_else(|| Vector::new([msg.as_bytes(), b"nft"].concat()));

            env::log(format!("Send NFT {} from {}", &token_id, &previous_owner_id).as_bytes());
            nft_trusts.push(&NftTrustTransaction {
                from_account_id: previous_owner_id,
                nft_contract_id: env::predecessor_account_id(),
                nft_token_id: token_id,
            });
            self.nft_trusts.insert(&msg, &nft_trusts);
        }
        false
    }

    pub fn ft_on_transfer(&mut self, sender_id: String, amount: U128, msg: String) -> U128 {
        let account_id = self.phone_to_user.get(&msg);

        if account_id.is_some() {
            ft_contract::ft_transfer(
                account_id.clone().unwrap(),
                amount,
                &env::predecessor_account_id(),
                1,
                GAS,
            );
        } else {
            let mut ft_trusts = self
                .ft_trusts
                .get(&msg)
                .unwrap_or_else(|| Vector::new([msg.as_bytes(), b"ft"].concat()));
            env::log(
                format!(
                    "Send FT {} from {}",
                    &env::predecessor_account_id(),
                    &sender_id
                )
                .as_bytes(),
            );

            ft_trusts.push(&FtTrustTransaction {
                from_account_id: sender_id,
                ft_contract_id: env::predecessor_account_id(),
                ft_amount: amount.into(),
            });
            self.ft_trusts.insert(&msg, &ft_trusts);
        }
        U128(0)
    }

    pub fn get_account_id(&self, phone: String) -> Option<String> {
        self.phone_to_user.get(&phone)
    }

    pub fn get_transfers(&self, phone: String) -> Option<Vec<NearTrustTransaction>> {
        let near_trusts = self.near_trusts.get(&phone);
        if near_trusts.is_some() {
            return Some(near_trusts.unwrap().to_vec());
        }
        None
    }

    pub fn get_ft_transfers(&self, phone: String) -> Option<Vec<FtTrustTransaction>> {
        let ft_trusts = self.ft_trusts.get(&phone);
        if ft_trusts.is_some() {
            return Some(ft_trusts.unwrap().to_vec());
        }
        None
    }

    pub fn get_nft_transfers(&self, phone: String) -> Option<Vec<NftTrustTransaction>> {
        let nft_trusts = self.nft_trusts.get(&phone);
        if nft_trusts.is_some() {
            return Some(nft_trusts.unwrap().to_vec());
        }
        None
    }
}
