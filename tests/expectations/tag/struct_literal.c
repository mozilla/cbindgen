#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Bar;

struct Foo {
  int32_t a;
  uint32_t b;
};
#define Foo_FOO (Foo){ .a = 42, .b = 47 }


#define BAR (Foo){ .a = 42, .b = 1337 }



void root(struct Foo x, struct Bar bar);
