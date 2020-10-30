#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define FOUR 4

enum E
#ifdef __cplusplus
  : int8_t
#endif // __cplusplus
 {
  A = 1,
  B = -1,
  C = (1 + 2),
  D = FOUR,
  F = 5,
  G = (int8_t)'6',
  H = (int8_t)false,
};
#ifndef __cplusplus
typedef int8_t E;
#endif // __cplusplus

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const E*);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
