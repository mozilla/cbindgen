#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct PREFIXFoo {
  int32_t a;
  uint32_t b;
};

#define PREFIXBAR (PREFIXFoo){ .a = 42, .b = 1337 }

#define PREFIXFoo_FOO (PREFIXFoo){ .a = 42, .b = 47 }

void root(struct PREFIXFoo x);
