ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

build:
	rm -rf build/
	mkdir -p build/
	cd lib/hello && cargo build --release
	cp lib/hello/target/release/hello.dll.lib build/
	cp lib/hello/target/release/hello.dll build/
	cd ../../
	go build -ldflags="-r $(ROOT_DIR)build" main.go
	mv main.exe build/

run: build
	./main
