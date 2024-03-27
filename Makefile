.PHONY: build run debug clean

build:
	cargo build

run: build
	cargo run

debug: build
	cargo run --verbose

clean:
	cargo clean