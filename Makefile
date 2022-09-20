VERISON=$(shell grep "version" Cargo.toml | cut -d' ' -f3 | cut -d'"' -f2)

default: format test build_windows build_linux

format:
	rustup component add rustfmt
	cargo fmt -- --emit=files

build_windows:
	cargo build --target=x86_64-pc-windows-gnu --release
	mkdir -p target/build/$(VERISON)/windows/assets
	cp -rp assets/ target/build/$(VERISON)/windows/assets/ 
	cp target/release/bevy-test target/build/$(VERISON)/windows/assets/bevy-test

build_linux:
	cargo build --release
	mkdir -p target/build/$(VERISON)/linux/assets
	cp -rp assets/ target/build/$(VERISON)/linux/assets/
	cp target/release/bevy-test target/build/$(VERISON)/linux/assets/bevy-test

test:
	cargo test