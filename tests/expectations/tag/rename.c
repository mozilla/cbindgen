#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

#define C_H 10

enum C_E {
  x = 0,
  y = 1,
};
typedef uint8_t C_E;

struct C_A;

struct C_C;

struct C_AwesomeB {
  int32_t x;
  float y;
};

union C_D {
  int32_t x;
  float y;
};

typedef struct C_A C_F;

extern const int32_t G;

void root(const struct C_A *a, struct C_AwesomeB b, struct C_C c, union C_D d, C_E e, C_F f);
