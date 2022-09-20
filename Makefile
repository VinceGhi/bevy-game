VERISON=$(grep "version" Cargo.toml | cut -d' ' -f3 | cut -d'"' -f2)

default: build

.PHONY: build-build
build: format build_windows build_linux mkdir copy_assets copy_binary_linux copy_binary_windows 

.PHONY: build_windows-build
build_windows: format build_windows mkdir copy_assets copy_binary_windows 

.PHONY: build_linux-build
build_linux: format build_linux mkdir copy_assets copy_binary_linux 

.PHONY: docker-build
format:
	rustup component add rustfmt 2>/dev/null && cargo fmt -- --emit=files

.PHONY: build_windows-build
build_windows:
	cargo build --target=x86_64-pc-windows-gnu --release

.PHONY: build_linux-build
build_linux:
	cargo build --release

.PHONY: mkdir-build
mkdir:
	mkdir -p target/build/$(VERISON)/assets

.PHONY: copy_assets-build
copy_assets:
	cp -rp assets/ target/build/$(VERISON)/assets/

.PHONY: copy_binary_linux-build
copy_binary_linux:
	cp target/release/bevy-test target/build/$(VERISON)/assets/bevy-test

.PHONY: copy_binary_windows-build
copy_binary_windows:
	cp target/x86_64-pc-windows-gnu/release/bevy-test.exe target/build/$(VERISON)/assets/bevy-test.exe

.PHONY: docker-test
test:
	command = "cargo"
	args = ["test"]