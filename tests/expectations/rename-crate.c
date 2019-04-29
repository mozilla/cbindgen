#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  uint64_t y;
} RenamedTy;

typedef struct {
  int32_t x;
} Foo;

void renamed_func(RenamedTy a);

void root(Foo a);
