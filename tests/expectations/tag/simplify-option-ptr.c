#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

struct Opaque;

struct Foo {
  const struct Opaque *x;
  struct Opaque *y;
  void (*z)();
};

union Bar {
  const struct Opaque *x;
  struct Opaque *y;
  void (*z)();
};

void root(const struct Opaque *a, struct Opaque *b, struct Foo c, union Bar d);
