.PHONY: check run install reinstall clean

check:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test --all

run:
	cargo run

install:
	cargo install --path .

reinstall:
	cargo install --path . --force

clean:
	cargo clean
