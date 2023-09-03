
#include <stdbool.h>
        

// std->io ------------

#include <stdio.h>
#include <stdlib.h>

void io___out___in(int x) { printf("%d\n", x); }
void io___out___str(char* x) { printf("%s\n", x); }
void io___out___bool(bool x) {
  if (x == true) {
    io___out___str("true");
  } else {
    io___out___str("false");
  }
}
#define io___out(x)          \
  _Generic(x,                \
      int: io___out___in,    \
      char*: io___out___str, \
      bool: io___out___bool)(x)

int io___in___in() {
  int x = 0;
  scanf("%d", &x);
  return x;
}
char* io___in___str() {
  char* x = NULL;
  int ch;
  int capacity = 10, size = 0;

  x = realloc(x, sizeof(*x) * (capacity + 1));
  if (!x) return NULL;
  while ((ch = getchar()) != EOF && ch != '\n') {
    if (size == capacity) {
      capacity += 10;
      x = realloc(x, sizeof(*x) * (capacity + 1));
      if (!x) return NULL;
    }
    x[size] = (char)ch;
    size++;
  }
  x[size] = '\0';

  return x;
}
bool io__in__bool() {
  int x = io___in___in();
  if (x == 0) {
    return false;
  }
  return true;
}

#define io___in(x) \
  x = _Generic(x, int: io___in___in, char*: io___in___str, bool: io__in__bool)()

// --------------------
    


// std->string ------------

#include <stdarg.h>

// source code of sprintf()
char* string___format(
    char const* const _Format,
    ...
) {
    char* s;
    va_list _ArgList;
    _crt_va_start(_ArgList, _Format);

    _vsprintf_l(s, _Format, NULL, _ArgList);

    _crt_va_end(_ArgList);

    return s;
}

// ------------------------
   
int main(int _argc, char* _argv[]) {
char* s = "";io___out("Enter string: ");io___in(s);io___out("You entered %d"s);return 0;}