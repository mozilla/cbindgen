#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum C {
  X = 2,
  Y,
};
typedef uint32_t C;

struct A {
  int32_t _0;
};

struct B {
  int32_t x;
  float y;
};

struct D {
  uint8_t List;
  uintptr_t Of;
  struct B Things;
};

void root(struct A a, struct B b, C c, struct D d);
