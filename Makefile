fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings


test:
	cargo test

check: fmt clippy test