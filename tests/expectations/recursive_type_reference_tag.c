#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct B;

struct A {
  struct B *buf;
  uintptr_t len;
};

struct B {
  int32_t something;
  struct A nested;
};

void root(const struct B *foo);
