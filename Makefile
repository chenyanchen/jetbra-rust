.PHONY: build dist

all: build

build:
	cargo build --release --target aarch64-apple-darwin
	cargo build --release --target x86_64-apple-darwin

VERSION = $(shell git describe --tags --always --dirty)

dist: build
	mkdir -p dist && rm -r dist/*

	cp target/aarch64-apple-darwin/release/jetbra jetbra
	tar czf dist/jetbra-$(VERSION)-aarch64-apple-darwin.tar.gz jetbra

	cp target/x86_64-apple-darwin/release/jetbra jetbra
	tar czf dist/jetbra-$(VERSION)-x86_64-apple-darwin.tar.gz jetbra

	lipo -create -output jetbra target/*-apple-darwin/release/jetbra
	tar czf dist/jetbra-$(VERSION)-universal-apple-darwin.tar.gz jetbra

	rm jetbra

	shasum -a 256 dist/* > dist/checksums.txt
