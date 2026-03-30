#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum C
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint32_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  X = 2,
  Y,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum C C;
#else
typedef uint32_t C;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

typedef struct A {
  int32_t m0;
} A;

typedef struct B {
  int32_t x;
  float y;
} B;

enum F_Tag
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Foo,
  Bar,
  Baz,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum F_Tag F_Tag;
#else
typedef uint8_t F_Tag;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

typedef struct Bar_Body {
  F_Tag tag;
  uint8_t x;
  int16_t y;
} Bar_Body;

typedef union F {
  F_Tag tag;
  struct {
    F_Tag foo_tag;
    int16_t foo;
  };
  Bar_Body bar;
} F;

enum H_Tag
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Hello,
  There,
  Everyone,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum H_Tag H_Tag;
#else
typedef uint8_t H_Tag;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

typedef struct There_Body {
  uint8_t x;
  int16_t y;
} There_Body;

typedef struct H {
  H_Tag tag;
  union {
    struct {
      int16_t hello;
    };
    There_Body there;
  };
} H;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct A x, struct B y, C z, union F f, struct H h);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
