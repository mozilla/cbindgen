#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

struct Opaque;

union Normal {
  int32_t x;
  float y;
};

union NormalWithZST {
  int32_t x;
  float y;
};

void root(struct Opaque *a, union Normal b, union NormalWithZST c);
