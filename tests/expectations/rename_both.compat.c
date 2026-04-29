#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define C_H 10

enum C_E
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  x = 0,
  y = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum C_E C_E;
#else
typedef uint8_t C_E;
#endif // __STDC_VERSION__ >= 202311L
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

typedef struct C_A C_F;

#define C_I (intptr_t)(C_F*)10

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern const int32_t G;

void root(const struct C_A *a, struct C_AwesomeB b, struct C_C c, union C_D d, C_E e, C_F f);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
