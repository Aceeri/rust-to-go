#include <stdio.h>
#include "gateway.h"
#include "interfacerust.h"

void gateway_register(callback_fn callback) {
	int response = callback("test");
	printf("Response: %d\n", response);
	interface_register("nice", callback);
}
