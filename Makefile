all: install format lint test

install:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

clean:
	rm -rf $(VENV)

install:
	cargo build 

format:
	cargo fmt

lint:
	 cargo clippy --all-targets --

test:
	cargo test 

run:
	cargo run 

build:
	cargo build --release