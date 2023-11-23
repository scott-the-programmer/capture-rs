.PHONY: build run clean

build:
	cargo build

release:
	cargo build --release

run:
	cargo run

clean:
	cargo clean

test:
	cargo test

fix:
	cargo fix --allow-dirty --allow-staged
