#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define FOUR 4

enum E {
  A = 1,
  B = -1,
  C = (1 + 2),
  D = FOUR,
  F = 5,
};
typedef int8_t E;

enum E_NoCython {
  G = (int8_t)'6',
  H = (int8_t)false,
};
typedef int8_t E_NoCython;

void root(const E*);

void root_no_cython(const E_NoCython*);
