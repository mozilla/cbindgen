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

typedef enum L {
  l1,
  l2,
  l3,
  l4,
} L;

enum M {
  m1 = -1,
  m2 = 0,
  m3 = 1,
};
typedef int8_t M;

typedef enum N {
  n1,
  n2,
  n3,
  n4,
} N;

enum O {
  o1,
  o2,
  o3,
  o4,
};
typedef int8_t O;

typedef struct J J;

typedef struct K K;

typedef struct Opaque Opaque;

enum G_Tag {
  Foo,
  Bar,
  Baz,
};
typedef uint8_t G_Tag;

typedef struct Foo_Body {
  G_Tag tag;
  int16_t _0;
} Foo_Body;

typedef struct Bar_Body {
  G_Tag tag;
  uint8_t x;
  int16_t y;
} Bar_Body;

typedef union G {
  G_Tag tag;
  Foo_Body foo;
  Bar_Body bar;
} G;

typedef enum H_Tag {
  H_Foo,
  H_Bar,
  H_Baz,
} H_Tag;

typedef struct H_Foo_Body {
  int16_t _0;
} H_Foo_Body;

typedef struct H_Bar_Body {
  uint8_t x;
  int16_t y;
} H_Bar_Body;

typedef struct H {
  H_Tag tag;
  union {
    H_Foo_Body foo;
    H_Bar_Body bar;
  };
} H;

enum I_Tag {
  I_Foo,
  I_Bar,
  I_Baz,
};
typedef uint8_t I_Tag;

typedef struct I_Foo_Body {
  int16_t _0;
} I_Foo_Body;

typedef struct I_Bar_Body {
  uint8_t x;
  int16_t y;
} I_Bar_Body;

typedef struct I {
  I_Tag tag;
  union {
    I_Foo_Body foo;
    I_Bar_Body bar;
  };
} I;

void root(Opaque *opaque,
          A a,
          B b,
          C c,
          D d,
          E e,
          F f,
          G g,
          H h,
          I i,
          J j,
          K k,
          L l,
          M m,
          N n,
          O o);
