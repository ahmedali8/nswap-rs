RFLAGS="-C link-arg=-s"

rust-setup:
	./scripts/rust_setup.sh

build: build-nswap

build-nswap: nswap
	rustup target add wasm32-unknown-unknown
	RUSTFLAGS=$(RFLAGS) cargo build -p nswap --target wasm32-unknown-unknown --release
	mkdir -p build
	cp target/wasm32-unknown-unknown/release/nswap.wasm ./build/nswap.wasm

test: test-nswap

test-nswap: build-nswap mock-ft mock-rated
	RUSTFLAGS=$(RFLAGS) cargo test -p nswap

mock-ft: test-token
	rustup target add wasm32-unknown-unknown
	RUSTFLAGS=$(RFLAGS) cargo build -p test-token --target wasm32-unknown-unknown --release
	mkdir -p build
	cp target/wasm32-unknown-unknown/release/test_token.wasm ./build/test_token.wasm

mock-rated: test-rated-token
	rustup target add wasm32-unknown-unknown
	RUSTFLAGS=$(RFLAGS) cargo build -p test-rated-token --target wasm32-unknown-unknown --release
	mkdir -p build
	cp target/wasm32-unknown-unknown/release/test_rated_token.wasm ./build/test_rated_token.wasm

clean:
	cargo clean
	rm -rf build/
