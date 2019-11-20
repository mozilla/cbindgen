#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <uchar.h>

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

typedef enum {
  k1,
  k2,
  k3,
  k4,
} K;

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

typedef struct I I;

typedef struct J J;

typedef struct Opaque Opaque;

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

typedef struct {
  F_Tag tag;
  int16_t _0;
} Foo_Body;

typedef struct {
  F_Tag tag;
  uint8_t x;
  int16_t y;
} Bar_Body;

typedef union {
  F_Tag tag;
  Foo_Body foo;
  Bar_Body bar;
} F;

typedef enum {
  G_Foo,
  G_Bar,
  G_Baz,
} G_Tag;

typedef struct {
  int16_t _0;
} G_Foo_Body;

typedef struct {
  uint8_t x;
  int16_t y;
} G_Bar_Body;

typedef struct {
  G_Tag tag;
  union {
    G_Foo_Body foo;
    G_Bar_Body bar;
  };
} G;

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

typedef struct {
  int16_t _0;
} H_Foo_Body;

typedef struct {
  uint8_t x;
  int16_t y;
} H_Bar_Body;

typedef struct {
  H_Tag tag;
  union {
    H_Foo_Body foo;
    H_Bar_Body bar;
  };
} H;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(Opaque *o, A a, B b, C c, D d, E e, F f, G g, H h, I i, J j, K k, L l);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
