#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

struct A {
  int32_t x;
  float y;
};

struct B {
  struct A data;
};
