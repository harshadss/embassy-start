clean:
	cargo clean

build-dev: clean
	cargo build --verbose

build: clean
	cargo build

size-dev:
	cargo size -v --target thumbv7em-none-eabihf --bin embassy-start -- -A

size:
	cargo size -v --target thumbv7em-none-eabihf --bin embassy-start --release -- -A