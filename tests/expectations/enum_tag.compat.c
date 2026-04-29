#if 0
''' '
#endif

#ifdef __cplusplus
template <typename T>
using Box = T*;
#endif

#if 0
' '''
#endif


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum A
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint64_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  a1 = 0,
  a2 = 2,
  a3,
  a4 = 5,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum A A;
#else
typedef uint64_t A;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum B
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint32_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  b1 = 0,
  b2 = 2,
  b3,
  b4 = 5,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum B B;
#else
typedef uint32_t B;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum C
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  c1 = 0,
  c2 = 2,
  c3,
  c4 = 5,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum C C;
#else
typedef uint16_t C;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum D
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  d1 = 0,
  d2 = 2,
  d3,
  d4 = 5,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum D D;
#else
typedef uint8_t D;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum E
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uintptr_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  e1 = 0,
  e2 = 2,
  e3,
  e4 = 5,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum E E;
#else
typedef uintptr_t E;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum F
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : intptr_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  f1 = 0,
  f2 = 2,
  f3,
  f4 = 5,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum F F;
#else
typedef intptr_t F;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum L {
  l1,
  l2,
  l3,
  l4,
};

enum M
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : int8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  m1 = -1,
  m2 = 0,
  m3 = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum M M;
#else
typedef int8_t M;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum N {
  n1,
  n2,
  n3,
  n4,
};

enum O
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : int8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  o1,
  o2,
  o3,
  o4,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum O O;
#else
typedef int8_t O;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

struct J;

struct K;

struct Opaque;

enum G_Tag
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
typedef enum G_Tag G_Tag;
#else
typedef uint8_t G_Tag;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

struct Bar_Body {
  G_Tag tag;
  uint8_t x;
  int16_t y;
};

union G {
  G_Tag tag;
  struct {
    G_Tag foo_tag;
    int16_t foo;
  };
  struct Bar_Body bar;
};

enum H_Tag {
  H_Foo,
  H_Bar,
  H_Baz,
};

struct H_Bar_Body {
  uint8_t x;
  int16_t y;
};

struct H {
  enum H_Tag tag;
  union {
    struct {
      int16_t foo;
    };
    struct H_Bar_Body bar;
  };
};

enum ExI_Tag
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  ExI_Foo,
  ExI_Bar,
  ExI_Baz,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum ExI_Tag ExI_Tag;
#else
typedef uint8_t ExI_Tag;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

struct ExI_Bar_Body {
  uint8_t x;
  int16_t y;
};

struct ExI {
  ExI_Tag tag;
  union {
    struct {
      int16_t foo;
    };
    struct ExI_Bar_Body bar;
  };
};

enum P_Tag
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  P0,
  P1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum P_Tag P_Tag;
#else
typedef uint8_t P_Tag;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

struct P1_Body {
  uint8_t _0;
  uint8_t _1;
  uint8_t _2;
};

struct P {
  P_Tag tag;
  union {
    struct {
      uint8_t p0;
    };
    struct P1_Body p1;
  };
};

enum Q_Tag {
  Ok,
  Err,
};

struct Q {
  enum Q_Tag tag;
  union {
    struct {
      uint32_t *ok;
    };
    struct {
      uint32_t err;
    };
  };
};

enum R_Tag {
  IRFoo,
  IRBar,
  IRBaz,
};

struct IRBar_Body {
  uint8_t x;
  int16_t y;
};

struct R {
  enum R_Tag tag;
  union {
    struct {
      int16_t IRFoo;
    };
    struct IRBar_Body IRBar;
  };
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct Opaque *opaque,
          A a,
          B b,
          C c,
          D d,
          E e,
          F f,
          union G g,
          struct H h,
          struct ExI i,
          struct J j,
          struct K k,
          enum L l,
          M m,
          enum N n,
          O o,
          struct P p,
          struct Q q,
          struct R r);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#if 0
''' '
#endif

#include <stddef.h>
#include "testing-helpers.h"
static_assert(offsetof(CBINDGEN_STRUCT(P), tag) == 0, "unexpected offset for tag");
static_assert(offsetof(CBINDGEN_STRUCT(P), p0) == 1, "unexpected offset for p0");
static_assert(offsetof(CBINDGEN_STRUCT(P), p0) == 1, "unexpected offset for p1");
static_assert(sizeof(CBINDGEN_STRUCT(P)) == 4, "unexpected size for P");

#if 0
' '''
#endif
