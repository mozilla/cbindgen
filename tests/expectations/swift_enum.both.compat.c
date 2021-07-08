#define CF_ENUM(_type, _name) _type _name; enum

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef CF_ENUM(uint64_t, A) {
  a1 = 0,
  a2 = 2,
  a3,
  a4 = 5,
};

typedef CF_ENUM(uint32_t, B) {
  b1 = 0,
  b2 = 2,
  b3,
  b4 = 5,
};

typedef CF_ENUM(uint16_t, C) {
  c1 = 0,
  c2 = 2,
  c3,
  c4 = 5,
};

typedef CF_ENUM(uint8_t, D) {
  d1 = 0,
  d2 = 2,
  d3,
  d4 = 5,
};

typedef CF_ENUM(uintptr_t, E) {
  e1 = 0,
  e2 = 2,
  e3,
  e4 = 5,
};

typedef CF_ENUM(intptr_t, F) {
  f1 = 0,
  f2 = 2,
  f3,
  f4 = 5,
};

typedef enum L {
  l1,
  l2,
  l3,
  l4,
} L;

typedef CF_ENUM(int8_t, M) {
  m1 = -1,
  m2 = 0,
  m3 = 1,
};

typedef enum N {
  n1,
  n2,
  n3,
  n4,
} N;

typedef CF_ENUM(int8_t, O) {
  o1,
  o2,
  o3,
  o4,
};

typedef struct J J;

typedef struct K K;

typedef struct Opaque Opaque;

typedef CF_ENUM(uint8_t, G_Tag) {
  Foo,
  Bar,
  Baz,
};

typedef struct Bar_Body {
  G_Tag tag;
  uint8_t x;
  int16_t y;
} Bar_Body;

typedef union G {
  G_Tag tag;
  struct {
    G_Tag foo_tag;
    int16_t foo;
  };
  Bar_Body bar;
} G;

typedef enum H_Tag {
  H_Foo,
  H_Bar,
  H_Baz,
} H_Tag;

typedef struct H_Bar_Body {
  uint8_t x;
  int16_t y;
} H_Bar_Body;

typedef struct H {
  H_Tag tag;
  union {
    struct {
      int16_t foo;
    };
    H_Bar_Body bar;
  };
} H;

typedef CF_ENUM(uint8_t, ExI_Tag) {
  ExI_Foo,
  ExI_Bar,
  ExI_Baz,
};

typedef struct ExI_Bar_Body {
  uint8_t x;
  int16_t y;
} ExI_Bar_Body;

typedef struct ExI {
  ExI_Tag tag;
  union {
    struct {
      int16_t foo;
    };
    ExI_Bar_Body bar;
  };
} ExI;

typedef CF_ENUM(uint8_t, P_Tag) {
  P0,
  P1,
};

typedef struct P1_Body {
  uint8_t _0;
  uint8_t _1;
  uint8_t _2;
} P1_Body;

typedef struct P {
  P_Tag tag;
  union {
    struct {
      uint8_t p0;
    };
    P1_Body p1;
  };
} P;

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
          struct P p);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
