#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct RenamedTy {
  uint64_t y;
};

struct Foo {
  int32_t x;
};

void renamed_func(struct RenamedTy a);

void root(struct Foo a);
