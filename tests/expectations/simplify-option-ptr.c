#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Opaque Opaque;

typedef struct {
  const Opaque *x;
  Opaque *y;
  void (*z)(void);
} Foo;

typedef union {
  const Opaque *x;
  Opaque *y;
  void (*z)(void);
} Bar;

void root(const Opaque *a, Opaque *b, Foo c, Bar d);
