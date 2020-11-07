init: toolchain build-release

toolchain:
	./scripts/init.sh

check:
	SKIP_WASM_BUILD= cargo check

test:
	SKIP_WASM_BUILD= cargo test --all

run-debug:
	WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo run -- --dev --tmp -lruntime=debug

run:
	WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo run -- --dev

purge:
	WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo run -- purge-chain --dev -y

build-debug:
	WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo build

build-release:
	WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo build --release
