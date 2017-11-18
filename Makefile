ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))
BUILD_DIR := $(ROOT_DIR)build/

build: clean
	cd lib/hello && cargo build --release
	cp lib/hello/target/release/libhello.d $(BUILD_DIR)
	cp lib/hello/target/release/libhello.so $(BUILD_DIR)
	cp lib/hello.h $(BUILD_DIR)
	cd ../../
	go build -ldflags="-r $(BUILD_DIR)" main.go
	mv main build/

clean:
	rm -rf $(BUILD_DIR)
	mkdir -p $(BUILD_DIR)

run: build
	.$(BUILD_DIR)main
