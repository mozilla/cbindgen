#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct A {
  int32_t namespace_;
  float float_;
} A;

typedef struct B {
  int32_t namespace_;
  float float_;
} B;

enum C_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  D,
};
#ifndef __cplusplus
typedef uint8_t C_Tag;
#endif // __cplusplus

typedef struct D_Body {
  int32_t namespace_;
  float float_;
} D_Body;

typedef struct C {
  C_Tag tag;
  union {
    D_Body d;
  };
} C;

enum E_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  Double,
  Float,
};
#ifndef __cplusplus
typedef uint8_t E_Tag;
#endif // __cplusplus

typedef struct Double_Body {
  double _0;
} Double_Body;

typedef struct Float_Body {
  float _0;
} Float_Body;

typedef struct E {
  E_Tag tag;
  union {
    Double_Body double_;
    Float_Body float_;
  };
} E;

enum F_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  double_,
  float_,
};
#ifndef __cplusplus
typedef uint8_t F_Tag;
#endif // __cplusplus

typedef struct double_Body {
  double _0;
} double_Body;

typedef struct float_Body {
  float _0;
} float_Body;

typedef struct F {
  F_Tag tag;
  union {
    double_Body double_;
    float_Body float_;
  };
} F;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(A a, B b, C c, E e, F f, int32_t namespace_, float float_);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
