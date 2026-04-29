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
  A_Sentinel,
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
  B_Sentinel,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum B B;
#else
typedef uint8_t B;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum C_Tag
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  C_C1,
  C_C2,
  C_C3,
  /**
   * Must be last for serialization purposes
   */
  C_Sentinel,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum C_Tag C_Tag;
#else
typedef uint8_t C_Tag;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

typedef struct C_C1_Body {
  C_Tag tag;
  uint32_t a;
} C_C1_Body;

typedef struct C_C2_Body {
  C_Tag tag;
  uint32_t b;
} C_C2_Body;

typedef union C {
  C_Tag tag;
  C_C1_Body c1;
  C_C2_Body c2;
} C;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(A a, B b, union C c);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
