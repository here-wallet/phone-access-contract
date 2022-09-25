DEV_CONTRACT=phone.herewallet.testnet

build:
	cd contract && cargo build --target wasm32-unknown-unknown --release && cd ../ && \
	cp contract/target/wasm32-unknown-unknown/release/here_phone.wasm ./out/main.wasm

test:
	cd contract && RUST_BACKTRACE=1 cargo test && cd ..


deploy-dev:
	make build && \
	near deploy phone.herewallet.testnet

# near delete phone.herewallet.testnet herewallet.testnet

deploy-prod:
	make build && \
	NEAR_ENV=mainnet near deploy phone.herewallet.near


run-test:
	cd contract && cargo build --target wasm32-unknown-unknown --release && cd ../ && \
	cp contract/target/wasm32-unknown-unknown/release/here_src.wasm ./out/main.wasm && \
	NEAR_ENV=mainnet near deploy bridge.herewallet.near


dev-init:
	near call $(DEV_CONTRACT) new '{"owner_id":"herewallet.testnet"}' --accountId herewallet.testnet  --gas 242794783120800    


init:
	NEAR_ENV=mainnet near call phone.herewallet.near new '{"owner_id":"herewallet.near"}' --accountId herewallet.near  --gas 242794783120800    
