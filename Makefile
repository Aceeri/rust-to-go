ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))
BUILD_DIR := $(ROOT_DIR)build/

build: clean
	gcc -c lib/gateway.c -o $(BUILD_DIR)gateway.o
	cd $(BUILD_DIR) && ar cru libgateway.a gateway.o
	cd lib/gateway && cargo build --release
	cp lib/gateway/target/release/libinterfacerust.d $(BUILD_DIR)
	cp lib/gateway/target/release/libinterfacerust.so $(BUILD_DIR)
	cp lib/gateway.h $(BUILD_DIR)
	go build -o "$(ROOT_DIR)build" -ldflags="-r $(BUILD_DIR)" $(ROOT_DIR)/lib/go/main.go

clean:
	cd lib/gateway && cargo clean
	rm -rf build/
	mkdir -p build/ 

run: build
	./build/main
