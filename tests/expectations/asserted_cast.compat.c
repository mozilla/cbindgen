#define MY_ASSERT(...) do { } while (0)
#define MY_ATTRS __attribute((noinline))


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct I I;

enum H_Tag
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  H_Foo,
  H_Bar,
  H_Baz,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum H_Tag H_Tag;
#else
typedef uint8_t H_Tag;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

typedef struct {
  uint8_t x;
  int16_t y;
} H_Bar_Body;

typedef struct {
  H_Tag tag;
  union {
    struct {
      int16_t foo;
    };
    H_Bar_Body bar;
  };
} H;

enum J_Tag
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  J_Foo,
  J_Bar,
  J_Baz,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum J_Tag J_Tag;
#else
typedef uint8_t J_Tag;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

typedef struct {
  uint8_t x;
  int16_t y;
} J_Bar_Body;

typedef struct {
  J_Tag tag;
  union {
    struct {
      int16_t foo;
    };
    J_Bar_Body bar;
  };
} J;

enum K_Tag
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  K_Foo,
  K_Bar,
  K_Baz,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum K_Tag K_Tag;
#else
typedef uint8_t K_Tag;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

typedef struct {
  K_Tag tag;
  uint8_t x;
  int16_t y;
} K_Bar_Body;

typedef union {
  K_Tag tag;
  struct {
    K_Tag foo_tag;
    int16_t foo;
  };
  K_Bar_Body bar;
} K;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void foo(H h, I i, J j, K k);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
