#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define C_H 10

enum C_E {
  x = 0,
  y = 1,
};
typedef uint8_t C_E;

typedef struct C_A C_A;

typedef struct C_C C_C;

typedef struct C_AwesomeB {
  int32_t x;
  float y;
} C_AwesomeB;

typedef union C_D {
  int32_t x;
  float y;
} C_D;

typedef C_A C_F;

extern const int32_t G;

void root(const C_A *a, C_AwesomeB b, C_C c, C_D d, C_E e, C_F f);
