#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <uchar.h>

#define C_H 10

enum C_E
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  x = 0,
  y = 1,
};
#ifndef __cplusplus
typedef uint8_t C_E;
#endif // __cplusplus

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

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern const int32_t G;

void root(const struct C_A *a, struct C_AwesomeB b, struct C_C c, union C_D d, C_E e, C_F f);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
