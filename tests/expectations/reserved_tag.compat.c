#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct A {
  int32_t namespace_;
  float float_;
};

struct B {
  int32_t namespace_;
  float float_;
};

enum C_Tag
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  D,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum C_Tag C_Tag;
#else
typedef uint8_t C_Tag;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

struct D_Body {
  int32_t namespace_;
  float float_;
};

struct C {
  C_Tag tag;
  union {
    struct D_Body d;
  };
};

enum E_Tag
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Double,
  Float,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum E_Tag E_Tag;
#else
typedef uint8_t E_Tag;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

struct E {
  E_Tag tag;
  union {
    struct {
      double double_;
    };
    struct {
      float float_;
    };
  };
};

enum F_Tag
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  double_,
  float_,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum F_Tag F_Tag;
#else
typedef uint8_t F_Tag;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

struct F {
  F_Tag tag;
  union {
    struct {
      double double_;
    };
    struct {
      float float_;
    };
  };
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct A a,
          struct B b,
          struct C c,
          struct E e,
          struct F f,
          int32_t namespace_,
          float float_);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
