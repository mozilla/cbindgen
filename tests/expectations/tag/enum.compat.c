#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum A
#ifdef __cplusplus
  : uint32_t
#endif // __cplusplus
 {
  a1 = 0,
  a2 = 2,
  a3,
  a4 = 5,
};
#ifndef __cplusplus

typedef uint32_t A;
#endif // __cplusplus


enum B
#ifdef __cplusplus
  : uint16_t
#endif // __cplusplus
 {
  b1 = 0,
  b2 = 2,
  b3,
  b4 = 5,
};
#ifndef __cplusplus

typedef uint16_t B;
#endif // __cplusplus


enum C
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  c1 = 0,
  c2 = 2,
  c3,
  c4 = 5,
};
#ifndef __cplusplus

typedef uint8_t C;
#endif // __cplusplus


enum D
#ifdef __cplusplus
  : uintptr_t
#endif // __cplusplus
 {
  d1 = 0,
  d2 = 2,
  d3,
  d4 = 5,
};
#ifndef __cplusplus

typedef uintptr_t D;
#endif // __cplusplus


enum E
#ifdef __cplusplus
  : intptr_t
#endif // __cplusplus
 {
  e1 = 0,
  e2 = 2,
  e3,
  e4 = 5,
};
#ifndef __cplusplus

typedef intptr_t E;
#endif // __cplusplus


enum K {
  k1,
  k2,
  k3,
  k4,
};
#ifndef __cplusplus

#endif // __cplusplus


enum L
#ifdef __cplusplus
  : int8_t
#endif // __cplusplus
 {
  l1 = -1,
  l2 = 0,
  l3 = 1,
};
#ifndef __cplusplus

typedef int8_t L;
#endif // __cplusplus


struct I;

struct J;

struct Opaque;

enum F_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  Foo,
  Bar,
  Baz,
};
#ifndef __cplusplus

typedef uint8_t F_Tag;
#endif // __cplusplus


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
#ifndef __cplusplus

#endif // __cplusplus


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

enum H_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  H_Foo,
  H_Bar,
  H_Baz,
};
#ifndef __cplusplus

typedef uint8_t H_Tag;
#endif // __cplusplus


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

#ifdef __cplusplus

extern "C" {

#endif // __cplusplus

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

#ifdef __cplusplus

} // extern "C"

#endif // __cplusplus
