

near call phone.herewallet.testnet send_near_to_phone '{"phone":"test"}' --gas 242794783120800 --accountId petr4.testnet --deposit 1
near call phone.herewallet.testnet receive_payments '{"phone":"test3"}' --gas 242794783120800 --accountId petr4.testnet
near call phone.herewallet.testnet allocate_phone '{"phone":"test3", "account_id":"petr4.testnet"}' --gas 242794783120800 --accountId herewallet.testnet --depositYocto 1

near call phone.herewallet.testnet allocate_phone '{"phone":"test", "account_id":"petr4.testnet"}' --gas 242794783120800 --accountId herewallet.testnet --depositYocto 1

near call usdn.testnet ft_transfer_call '{"receiver_id": "phone.herewallet.testnet", "amount": "100000000", "msg": "test"}' --accountId petr4.testnet  --gas 242794783120800 --depositYocto 1

near call paras-token-v2.testnet nft_transfer_call '{"receiver_id": "phone.herewallet.testnet","token_id":"499:1", "msg": "test3"}' --accountId petr.testnet  --gas 242794783120800 --depositYocto 1

NEAR_ENV=mainnet  near call tonic_goblin.enleap.near nft_transfer_call '{"receiver_id": "phone.herewallet.near","token_id":"1780", "msg": "d2c0a24cf13a5fe3b2faadee72131561483daff1da0c7aadb3637208fe8ac049.8d317d8b1207dd8e598213b433be7e82e3fe2ec57129d7a1ca652945ac0a07eb"}' --accountId mydev.near  --gas 242794783120800 --depositYocto 1

near view phone.herewallet.testnet get_ft_transfers '{"phone": "test"}' --accountId petr4.testnet 
near view phone.herewallet.testnet get_nft_transfers '{"phone": "d2c0a24cf13a5fe3b2faadee72131561483daff1da0c7aadb3637208fe8ac049.8d317d8b1207dd8e598213b433be7e82e3fe2ec57129d7a1ca652945ac0a07eb"}' --accountId petr4.testnet 