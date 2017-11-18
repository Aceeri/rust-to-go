void hello(char* name);

typedef int (*go_callback)(char* input);
void register_action(char* action, go_callback callback); 
int call_action(char* action, char* input);
