#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Foo {
  int32_t x;
  float y;
} Foo;

typedef struct Bar {
  struct Foo data;
} Bar;
