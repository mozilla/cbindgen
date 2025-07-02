#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Foo {
  uint32_t field;
};
#define Foo_FIELD_RELATED_CONSTANT 0

struct Bar {
  uint32_t field;
};
#define Bar_FIELD_RELATED_CONSTANT 0

void root(struct Foo a, struct Bar b);
