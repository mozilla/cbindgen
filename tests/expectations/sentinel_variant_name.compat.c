#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum A
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  A_A1,
  A_A2,
  A_A3,
  /**
   * Must be last for serialization purposes
   */
  A_Terminator,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum A A;
#else
typedef uint8_t A;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum B
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  B_B1,
  B_B2,
  B_B3,
  /**
   * Must be last for serialization purposes
   */
  B_Last,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum B B;
#else
typedef uint8_t B;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(A a, B b);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
