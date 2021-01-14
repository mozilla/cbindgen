#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Opaque Opaque;

typedef struct Option_____Opaque Option_____Opaque;

typedef struct Option_______c_void Option_______c_void;

typedef struct Foo {
  const struct Opaque *x;
  struct Opaque *y;
  void (*z)(void);
  struct Option_______c_void *zz;
} Foo;

typedef union Bar {
  const struct Opaque *x;
  struct Opaque *y;
  void (*z)(void);
  struct Option_______c_void *zz;
} Bar;

void root(const struct Opaque *a,
          struct Opaque *b,
          struct Foo c,
          union Bar d,
          struct Option_____Opaque *e);
