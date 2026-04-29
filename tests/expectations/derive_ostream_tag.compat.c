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

struct A {
  int32_t _0;
};

struct B {
  int32_t x;
  float y;
};

struct D {
  uint8_t List;
  uintptr_t Of;
  struct B Things;
};

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

struct Bar_Body {
  F_Tag tag;
  uint8_t x;
  int16_t y;
};

union F {
  F_Tag tag;
  struct {
    F_Tag foo_tag;
    int16_t foo;
  };
  struct Bar_Body bar;
};

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

struct There_Body {
  uint8_t x;
  int16_t y;
};

struct H {
  H_Tag tag;
  union {
    struct {
      int16_t hello;
    };
    struct There_Body there;
  };
};

enum I_Tag
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  ThereAgain,
  SomethingElse,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum I_Tag I_Tag;
#else
typedef uint8_t I_Tag;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

struct ThereAgain_Body {
  uint8_t x;
  int16_t y;
};

struct I {
  I_Tag tag;
  union {
    struct ThereAgain_Body there_again;
  };
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct A a, struct B b, C c, struct D d, union F f, struct H h, struct I i);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
