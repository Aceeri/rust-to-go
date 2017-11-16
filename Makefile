ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

build:
	cd lib/hello && cargo build --release
	cp lib/hello/target/release/hello.dll.lib lib/
	cp lib/hello/target/release/hello.dll lib/
	cd ../../
	go build -ldflags="-r $(ROOT_DIR)lib" main.go
	mv main.exe lib/

run: build
	./main
