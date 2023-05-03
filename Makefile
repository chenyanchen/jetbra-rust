dist:
	cargo build --release --target aarch64-apple-darwin
	cargo build --release --target x86_64-apple-darwin
	mkdir -p target/apple-darwin/release
	# Create the universal executable
	lipo -create -output target/apple-darwin/release/jetbra target/aarch64-apple-darwin/release/jetbra target/x86_64-apple-darwin/release/jetbra
	# Build windows executable
	#cargo build --release --target x86_64-pc-windows-gnu

