#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum C
#ifdef __cplusplus
  : uint32_t
#endif // __cplusplus
 {
  X = 2,
  Y,
};
#ifndef __cplusplus
typedef uint32_t C;
#endif // __cplusplus

struct A {
  int32_t m0;
};

struct B {
  int32_t x;
  float y;
};

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

enum H_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  Hello,
  There,
  Everyone,
};
#ifndef __cplusplus
typedef uint8_t H_Tag;
#endif // __cplusplus

struct Hello_Body {
  int16_t _0;
};

struct There_Body {
  uint8_t x;
  int16_t y;
};

struct H {
  enum H_Tag tag;
  union {
    struct Hello_Body hello;
    struct There_Body there;
  };
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct A x, struct B y, C z, union F f, struct H h);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
