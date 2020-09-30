#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum C {
  X = 2,
  Y,
};
typedef uint32_t C;

typedef struct {
  int32_t _0;
} A;

typedef struct {
  int32_t x;
  float y;
} B;

typedef struct {
  uint8_t List;
  uintptr_t Of;
  B Things;
} D;

void root(A a, B b, C c, D d);
