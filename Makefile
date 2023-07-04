.PHONY: build dist

all: build

build:
	cargo build --release --target aarch64-apple-darwin
	cargo build --release --target x86_64-apple-darwin

UNIVERSAL_PATH := target/apple-darwin/release

dist: build
	mkdir -p $(UNIVERSAL_PATH)
	# Create universal binary
	lipo -create -output $(UNIVERSAL_PATH)/jetbra target/aarch64-apple-darwin/release/jetbra target/x86_64-apple-darwin/release/jetbra
	# TODO: Build windows executable
	#cargo build --release --target x86_64-pc-windows-gnu
