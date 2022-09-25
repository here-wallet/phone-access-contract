NEAR access by phone number
====

Smart contract for send NEAR, NFT, FT by sms

Sample send NFT

```
near call tonic_goblin.enleap.near nft_transfer_call '{"receiver_id": "phone.herewallet.near","token_id":"1780", "msg": "[phone number hash]"}' --accountId mydev.near  --gas 242794783120800 --depositYocto 1
```


Sample send NEAR

```
near call phone.herewallet.testnet send_near_to_phone '{"phone":"[phone number hash]"}' --gas 242794783120800 --accountId petr4.testnet --deposit 1

```


Sample send FT
```
near call usdn.testnet ft_transfer_call '{"receiver_id": "phone.herewallet.testnet", "amount": "100000000", "msg": "phone number hash]"}' --accountId petr4.testnet  --gas 242794783120800 --depositYocto 1
```

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

```
near call phone.herewallet.testnet send_near_to_phone '{"phone":"test"}' --gas 242794783120800 --accountId petr4.testnet --deposit 1
near call phone.herewallet.testnet receive_payments '{"phone":"test3"}' --gas 242794783120800 --accountId petr4.testnet
near call phone.herewallet.testnet allocate_phone '{"phone":"test3", "account_id":"petr4.testnet"}' --gas 242794783120800 --accountId herewallet.testnet --depositYocto 1
```