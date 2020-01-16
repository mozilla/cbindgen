#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Foo_Bar {
  const int32_t *something;
} Foo_Bar;

typedef union Bar {
  int32_t something;
  Foo_Bar subexpressions;
} Bar;

void root(Bar b);
