
#include <stdbool.h>
        
#include <stdio.h>
void io___out___in(int x) {
    printf("%d\n", x);
}
void io___out___str(char* x) {
    printf("%s\n", x);
}
#define io___out(x) _Generic(x, int: io___out___in, char*: io___out___str)(x)
    int main(int _argc, char* _argv[]) {
while (true) {io___out("Hello World");}return 0;}