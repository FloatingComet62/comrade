
#include <stdbool.h>
        
#include <stdio.h>
void io___out___in(int x) {
    printf("%d\n", x);
}
void io___out___str(char* x) {
    printf("%s\n", x);
}
#define io___out(x) _Generic(x, int: io___out___in, char*: io___out___str)(x)
    struct Account{int principal_amount;int interest_rate;int duration_in_years;};int main(int _argc, char* _argv[]) {
struct Account user = { 20000, 7, 10 };io___out(user.principal_amount);return 0;}