#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct Opaque Opaque;

typedef struct Normal {
  int32_t x;
  float y;
} Normal;

typedef struct NormalWithZST {
  int32_t x;
  float y;
} NormalWithZST;

typedef struct TupleRenamed {
  int32_t m0;
  float m1;
} TupleRenamed;

typedef struct TupleNamed {
  int32_t x;
  float y;
} TupleNamed;

void root(Opaque *a, Normal b, NormalWithZST c, TupleRenamed d, TupleNamed e);
