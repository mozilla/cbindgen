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

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern const int32_t G;

void root(const C_A *a, C_AwesomeB b, C_C c, C_D d, C_E e, C_F f);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
