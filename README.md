 NEAR storage for HERE wallet
 ====

Smart contract for liquid steaking. Used in the HERE wallet application to store user NEAR and generate passive income.

 ## Abstract

 The smart contract implement the NEP-141 spec (similar to ERC-20) to store user funds. In exchange for the NEAR, the user receives tokens of the wrapped NEAR. Most of the money held on the contract is stacking, but some is always left free. This allows the user to swap hNEAR to NEAR at any time without commission and use their funds. 

 For transactions within the wallet hNEAR is used. For transactions with third-party users first conversion hNEAR to NEAR and then transfer NEAR. 

 The _astostaking_ project is used for steaking. Users receive an accrual of interest to their hNEAR account on demand. The resulting income can be withdrawn at any time, turning it into NEAR.


## Control

__There are private methods for controlling steaking__

- `stake(&self, amount: U128)`
- `unstake(&self, amount: U128)`
- `deposit_stake(&self, amount: U128)`
- `withdraw_stake(&self, amount: U128)`

With them, owner have full access to the allocation of staking. The calls duplicate the management methods  "astro-stakers.poolv1.near". (`stake`, `unstake`, `deposit`, `withdraw`)



__And public methods for controlling steaking__

- `public_force_unstake(&mut self)`
- `public_force_withdraw(&self)`

With the help of them community can hijack the control. If the security on the storage balance drops below 5% you can call methods `public_force_unstake`, `public_force_withdraw`. This will lead to withdrawal of funds from "astro-stakers.poolv1.near" and freezing the staking control for 6 epochs. Also the first to find the problem will be paid a bonus of 1 hNEAR.


## Staking income 

### Owner

Owner can receive his dividends at any time, using method `receive_owner_dividends(amount)`. `amount` must be less than the total balance of all users and the amount of accumulated dividends. The method `get_account_total_balance()` on the _astostaking_ contract will be called to check the storage balance. If the check is successful, the owner has received on his account hNEAR.

### User

User can receive his dividends at any time, using method `receive_dividends`. User will receive hNEAR = `user_balance / 1000 * apy_value / ONE_YEAR_TS * (NOW_TS - last_accrual_ts)`. 0 <= `apy_value` <= 1000

----------

## Development

1. Install `rustup` via https://rustup.rs/
2. Run the following:

```
rustup default stable
rustup target add wasm32-unknown-unknown
```

### Testing

Contracts have unit tests

```
make run-test
```

### Compiling

You can build release version by running next scripts inside each contract folder:

```
make build
```

### Deploying to TestNet

To deploy to TestNet, you can use next command:
```
make deploy-dev
```

This will use contract ID from `Makefile`


## Bash API

near call $CONTRACT  register_account  '' --gas 242794783120800 --accountId $ACCOUNT

near call $CONTRACT  storage_deposit  '' --gas 242794783120800 --accountId $ACCOUNT --deposit 1

near call $CONTRACT  storage_withdraw  '{"amount": "1000000000000000000000000"}' --gas 242794783120800 --accountId $ACCOUNT --depositYocto 1

near call $CONTRACT  storage_withdraw  '{"amount": "1000000000000000000000000", "to_account_id":"petr.testnet"}' --gas 242794783120800 --accountId $ACCOUNT --depositYocto 1

near call $CONTRACT  receive_dividends '{"to_account_id":"petr.testnet"}' --accountId $ACCOUNT --gas 242794783120800
