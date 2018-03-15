#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

enum C {
  X = 2,
  Y,
};
typedef uint32_t C;

struct A {
  int32_t m0;
};

struct B {
  int32_t x;
  float y;
};

void root(struct A x, struct B y, C z);
