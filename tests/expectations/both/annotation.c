#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

enum C {
  X = 2,
  Y,
};
typedef uint32_t C;

typedef struct A {
  int32_t m0;
} A;

typedef struct B {
  int32_t x;
  float y;
} B;

void root(A x, B y, C z);
