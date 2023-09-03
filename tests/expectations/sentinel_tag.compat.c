#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum A
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
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
typedef uint8_t A;
#endif // __cplusplus

enum B
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
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
typedef uint8_t B;
#endif // __cplusplus

enum C_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
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
typedef uint8_t C_Tag;
#endif // __cplusplus

struct C_C1_Body {
  C_Tag tag;
  uint32_t a;
};

struct C_C2_Body {
  C_Tag tag;
  uint32_t b;
};

union C {
  C_Tag tag;
  struct C_C1_Body c1;
  struct C_C2_Body c2;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(A a, B b, union C c);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
