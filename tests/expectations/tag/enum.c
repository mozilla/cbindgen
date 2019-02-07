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
typedef uint32_t A;

enum B {
  b1 = 0,
  b2 = 2,
  b3,
  b4 = 5,
};
typedef uint16_t B;

enum C {
  c1 = 0,
  c2 = 2,
  c3,
  c4 = 5,
};
typedef uint8_t C;

enum D {
  d1 = 0,
  d2 = 2,
  d3,
  d4 = 5,
};
typedef uintptr_t D;

enum E {
  e1 = 0,
  e2 = 2,
  e3,
  e4 = 5,
};
typedef intptr_t E;

enum K {
  k1,
  k2,
  k3,
  k4,
};

enum L {
  l1 = -1,
  l2 = 0,
  l3 = 1,
};
typedef int8_t L;

struct I;

struct J;

struct Opaque;

enum F_Tag {
  Foo,
  Bar,
  Baz,
};
typedef uint8_t F_Tag;

struct Foo_Body {
  F_Tag tag;
  int16_t _0;
};

struct Bar_Body {
  F_Tag tag;
  uint8_t x;
  int16_t y;
};

union F {
  enum F_Tag tag;
  struct Foo_Body foo;
  struct Bar_Body bar;
};

enum G_Tag {
  G_Foo,
  G_Bar,
  G_Baz,
};

struct G_Foo_Body {
  int16_t _0;
};

struct G_Bar_Body {
  uint8_t x;
  int16_t y;
};

struct G {
  enum G_Tag tag;
  union {
    struct G_Foo_Body foo;
    struct G_Bar_Body bar;
  };
};

enum H_Tag {
  H_Foo,
  H_Bar,
  H_Baz,
};
typedef uint8_t H_Tag;

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

void root(struct Opaque *o,
          A a,
          B b,
          C c,
          D d,
          E e,
          union F f,
          struct G g,
          struct H h,
          struct I i,
          struct J j,
          enum K k,
          L l);
