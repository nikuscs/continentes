.PHONY: check fmt clippy test test-all build all

check:
	cargo check --all-targets

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets

test:
	cargo test

test-all:
	cargo test -- --ignored

build:
	cargo build --release

all: fmt clippy test
