package main

/*
#cgo LDFLAGS: -L ../../build -lgateway
#include "../../build/gateway.h"

int call_go_func_fwd(char* id);
*/
import "C"

//export call_go_func
func call_go_func(id *C.char) int {
	str := C.GoString(id)
	fmt.Printf("Go.call_go_func called with: %s\n", str)
	return 0
}

func main() {
	C.gateway_register((C.callback_fn)(unsafe.Pointer(C.call_go_func_fwd)))
}
