.PHONY: run build

run:
	cargo watch -x run

build:
	cargo build

fix:
	cargo fix

fmt:
	cargo fmt

test:
	cargo test