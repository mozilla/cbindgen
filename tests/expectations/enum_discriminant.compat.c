#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define FOURTY_FOUR 4

enum E
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : int8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  A = 1,
  B = -1,
  C = (1 + 2),
  D = FOURTY_FOUR,
  F = 5,
  G = (int8_t)54,
  H = (int8_t)false,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum E E;
#else
typedef int8_t E;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const E*);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
