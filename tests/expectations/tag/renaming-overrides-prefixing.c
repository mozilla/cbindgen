#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <uchar.h>

struct StyleA;

struct B {
  int32_t x;
  float y;
};

void root(const struct StyleA *a, struct B b);
