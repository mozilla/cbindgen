#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Opaque Opaque;

typedef struct Foo {
  const struct Opaque *x;
  struct Opaque *y;
  void (*z)(void);
} Foo;

typedef union Bar {
  const struct Opaque *x;
  struct Opaque *y;
  void (*z)(void);
} Bar;

void root(const struct Opaque *a, struct Opaque *b, struct Foo c, union Bar d);
