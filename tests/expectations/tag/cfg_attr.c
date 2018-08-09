#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

struct Normal {
  int32_t x;
  float y;
};

struct TupleRenamed {
  int32_t m0;
  float m1;
};

struct TupleNamed {
  int32_t x;
  float y;
};

void root(struct Normal b, struct TupleRenamed d, struct TupleNamed e);
