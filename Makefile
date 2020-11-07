init-node: node-toolchain run-node

node-toolchain:
	./scripts/init.sh

check-node:
	SKIP_WASM_BUILD= cargo check

test-node:
	SKIP_WASM_BUILD= cargo test --all

run-node:
	WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo run --release -- --dev --tmp

purge-node:
	WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo run --release -- purge-chain --dev -y

build-node:
	WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo build --release

run-sidecar:
	cd ./sidecar && yarn && yarn start

run-demo:
	cd ./hot-wallet && yarn && yarn start
