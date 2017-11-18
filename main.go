package main

/*
#cgo LDFLAGS: -L ./build -lhello
#include "./build/hello.h"
*/
import "C"

import (
	"fmt"
	"unsafe"
)

//export test_fn
func test_fn(id *C.char) int {
	fmt.Printf("I'm %s", C.GoString(id))
	return 0
}

var GlobalCatch = test_fn

func main() {
	C.hello(C.CString("World"))
	C.register_action(C.CString("action"), (*[0]byte)(unsafe.Pointer(&GlobalCatch)))
	C.call_action(C.CString("action"), C.CString("input"))
}
