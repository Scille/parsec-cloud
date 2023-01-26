build:
	maturin develop --release --features test-utils

test:
	cargo test --workspace --exclude=parsec

pytest: build
	pytest -n auto

check:
	cargo check --workspace
	cargo check --workspace --features use-sodiumoxide
	cargo check --target wasm32-unknown-unknown
