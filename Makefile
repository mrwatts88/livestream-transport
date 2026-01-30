.PHONY: signaling consumer test fmt clippy check

p:
	cargo run --bin producer -- p

s:
	cargo run --bin consumer -- s

c:
	cargo run --bin consumer -- c

test:
	cargo test

fmt:
	cargo fmt

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

check:
	cargo check --all-targets --all-features

ci:
	make fmt clippy check test

