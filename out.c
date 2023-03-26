
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
int list[] = { 1, 2, 3, 4, 5, 6, 7 };io___out(list[0]);return 0;}