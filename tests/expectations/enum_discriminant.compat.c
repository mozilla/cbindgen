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
};
#ifndef __cplusplus
typedef int8_t E;
#endif // __cplusplus

enum E_NoCython
#ifdef __cplusplus
  : int8_t
#endif // __cplusplus
 {
  G = (int8_t)'6',
  H = (int8_t)false,
};
#ifndef __cplusplus
typedef int8_t E_NoCython;
#endif // __cplusplus

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const E*);

void root_no_cython(const E_NoCython*);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
