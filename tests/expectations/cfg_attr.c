#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct {
  int32_t x;
  float y;
} Normal;

typedef struct {
  int32_t m0;
  float m1;
} TupleRenamed;

typedef struct {
  int32_t x;
  float y;
} TupleNamed;

void root(Normal b, TupleRenamed d, TupleNamed e);
