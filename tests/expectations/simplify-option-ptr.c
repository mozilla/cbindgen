#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

struct Opaque;
typedef struct Opaque Opaque;

typedef struct {
  const Opaque *x;
  Opaque *y;
  void (*z)();
} Foo;

typedef union {
  const Opaque *x;
  Opaque *y;
  void (*z)();
} Bar;

void root(const Opaque *a, Opaque *b, Foo c, Bar d);
