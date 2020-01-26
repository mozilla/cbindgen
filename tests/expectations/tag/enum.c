#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum A {
  a1 = 0,
  a2 = 2,
  a3,
  a4 = 5,
};
typedef uint64_t A;

enum B {
  b1 = 0,
  b2 = 2,
  b3,
  b4 = 5,
};
typedef uint32_t B;

enum C {
  c1 = 0,
  c2 = 2,
  c3,
  c4 = 5,
};
typedef uint16_t C;

enum D {
  d1 = 0,
  d2 = 2,
  d3,
  d4 = 5,
};
typedef uint8_t D;

enum E {
  e1 = 0,
  e2 = 2,
  e3,
  e4 = 5,
};
typedef uintptr_t E;

enum F {
  f1 = 0,
  f2 = 2,
  f3,
  f4 = 5,
};
typedef intptr_t F;

enum L {
  l1,
  l2,
  l3,
  l4,
};

enum M {
  m1 = -1,
  m2 = 0,
  m3 = 1,
};
typedef int8_t M;

enum N {
  n1,
  n2,
  n3,
  n4,
};

enum O {
  o1,
  o2,
  o3,
  o4,
};
typedef int8_t O;

struct J;

struct K;

struct Opaque;

enum G_Tag {
  Foo,
  Bar,
  Baz,
};
typedef uint8_t G_Tag;

struct Foo_Body {
  G_Tag tag;
  int16_t _0;
};

struct Bar_Body {
  G_Tag tag;
  uint8_t x;
  int16_t y;
};

union G {
  G_Tag tag;
  struct Foo_Body foo;
  struct Bar_Body bar;
};

enum H_Tag {
  H_Foo,
  H_Bar,
  H_Baz,
};

struct H_Foo_Body {
  int16_t _0;
};

struct H_Bar_Body {
  uint8_t x;
  int16_t y;
};

struct H {
  enum H_Tag tag;
  union {
    struct H_Foo_Body foo;
    struct H_Bar_Body bar;
  };
};

enum I_Tag {
  I_Foo,
  I_Bar,
  I_Baz,
};
typedef uint8_t I_Tag;

struct I_Foo_Body {
  int16_t _0;
};

struct I_Bar_Body {
  uint8_t x;
  int16_t y;
};

struct I {
  I_Tag tag;
  union {
    struct I_Foo_Body foo;
    struct I_Bar_Body bar;
  };
};

enum P_Tag {
  P0,
  P1,
};
typedef uint8_t P_Tag;

struct P0_Body {
  uint8_t _0;
};

struct P1_Body {
  uint8_t _0;
  uint8_t _1;
  uint8_t _2;
};

struct P {
  P_Tag tag;
  union {
    struct P0_Body p0;
    struct P1_Body p1;
  };
};

void root(struct Opaque *opaque,
          A a,
          B b,
          C c,
          D d,
          E e,
          F f,
          union G g,
          struct H h,
          struct I i,
          struct J j,
          struct K k,
          enum L l,
          M m,
          enum N n,
          O o,
          struct P p);

#include <stddef.h>
#include "testing-helpers.h"
static_assert(offsetof(CBINDGEN_STRUCT(P), tag) == 0, "unexpected offset for tag");
static_assert(offsetof(CBINDGEN_STRUCT(P), p0) == 1, "unexpected offset for p0");
static_assert(offsetof(CBINDGEN_STRUCT(P), p0) == 1, "unexpected offset for p1");
static_assert(sizeof(CBINDGEN_STRUCT(P)) == 4, "unexpected size for P");
