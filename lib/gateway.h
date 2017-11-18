#ifndef GATEWAY_H_
#define GATEWAY_H_

typedef int (*callback_fn)(char*);
void gateway_register(callback_fn);

#endif

