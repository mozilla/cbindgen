#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Foo {
  int32_t a;
  uint32_t b;
} Foo;

#define BAR (Foo){ .a = 42, .b = 1337 }

#define Foo_FOO (Foo){ .a = 42, .b = 47 }

void root(Foo x);
