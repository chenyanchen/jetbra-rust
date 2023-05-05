.PHONY: dist

BINARY_NAME := jetbra
UNIVERSAL_PATH := target/apple-darwin/release

dist:
	cargo build --release --target aarch64-apple-darwin
	cargo build --release --target x86_64-apple-darwin
	mkdir -p $(UNIVERSAL_PATH)
	# Create the universal executable
	lipo -create -output $(UNIVERSAL_PATH)/$(BINARY_NAME) target/aarch64-apple-darwin/release/$(BINARY_NAME) target/x86_64-apple-darwin/release/$(BINARY_NAME)
	# Build windows executable
	#cargo build --release --target x86_64-pc-windows-gnu

