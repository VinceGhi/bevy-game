VERISON=$(shell grep "version" Cargo.toml | cut -d' ' -f3 | cut -d'"' -f2)

default: format test build_windows build_linux

format:
	rustup component add rustfmt
	cargo fmt -- --emit=files

build_windows:
	cargo build --target=x86_64-pc-windows-gnu --release 2>/dev/null
	mkdir -p target/build/$(VERISON)/windows/
	cp -rp assets/ target/build/$(VERISON)/windows/ 
	cp target/x86_64-pc-windows-gnu/release/bevy-test.exe target/build/$(VERISON)/windows/bevy-test.exe

build_linux:
	cargo build --release 2>/dev/null
	mkdir -p target/build/$(VERISON)/linux/
	cp -rp assets/ target/build/$(VERISON)/linux/
	cp target/release/bevy-test target/build/$(VERISON)/linux/bevy-test

test:
	cargo test --verbose