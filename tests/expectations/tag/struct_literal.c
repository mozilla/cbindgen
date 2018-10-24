#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

struct Foo {
  int32_t a;
  uint32_t b;
};

#define BAR (Foo){ .a = 42, .b = 1337 }

#define Foo_FOO (Foo){ .a = 42, .b = 47 }

void root(struct Foo x);
