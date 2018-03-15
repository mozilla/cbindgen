#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct Opaque Opaque;

typedef union Normal {
  int32_t x;
  float y;
} Normal;

typedef union NormalWithZST {
  int32_t x;
  float y;
} NormalWithZST;

void root(Opaque *a, Normal b, NormalWithZST c);
