
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
int i = 0;while (i<5) {io___out("Hello World");i+=1;}return 0;}