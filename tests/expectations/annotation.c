#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

enum C {
  X = 2,
  Y = 3,
};
typedef uint32_t C;

typedef struct {
  int32_t m0;
} A;

typedef struct {
  int32_t x;
  float y;
} B;

void root(A x, B y, C z);
