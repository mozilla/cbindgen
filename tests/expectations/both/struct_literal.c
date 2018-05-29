#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct Foo {
  int32_t a;
  uint32_t b;
} Foo;

#define BAR (Foo){ .a = 42, .b = 1337 }

#define FOO (Foo){ .a = 42, .b = 47 }

void root(Foo x);
