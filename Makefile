.PHONY: build dist

all: build

build:
	cargo build --release --target aarch64-apple-darwin
	cargo build --release --target x86_64-apple-darwin

UNIVERSAL_PATH := target/apple-darwin/release

dist: build
	mkdir -p $(UNIVERSAL_PATH)
	# Create universal binary
	lipo -create -output $(UNIVERSAL_PATH)/jetbra target/*-apple-darwin/release/jetbra

	# Create .tar.gz
	mkdir -p dist
	tar czf dist/jetbra_macos.tar.gz $(UNIVERSAL_PATH)/jetbra

	# checksums.txt
	shasum -a 256 dist/* > dist/checksums.txt
