
#include <stdbool.h>
        
#include <stdio.h>
void io___out___in(int x) {
    printf("%d\n", x);
}
void io___out___str(char* x) {
    printf("%s\n", x);
}
#define io___out(x) _Generic(x, int: io___out___in, char*: io___out___str)(x)
    int fib(int x) {
if(x == 1) {return 0;}else if(x == 2) {return 1;}else {return fib(x-1)+fib(x-2);;}}int main() {
io___out(fib(10));return 0;}