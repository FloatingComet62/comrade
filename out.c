
#include <stdio.h>
void print_int(int x) {
    printf("%d\n", x);
}
void print_str(char* x) {
    printf("%s\n", x);
}
#define io___out(x) _Generic(x, int: print_int, char*: print_str)(x)
    enum Days {Days_Monday, Days_Tuesday, Days_Wednesday, Days_Thursday, Days_Friday, Days_Saturday, Days_Sunday, };int main() {
int day = Days_Monday;io___out(day);return 0;}