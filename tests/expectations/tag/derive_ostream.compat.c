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
  int32_t _0;
};

struct B {
  int32_t x;
  float y;
};

struct D {
  uint8_t List;
  uintptr_t Of;
  struct B Things;
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
  F_Tag tag;
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
  H_Tag tag;
  union {
    struct Hello_Body hello;
    struct There_Body there;
  };
};

enum J_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  Hi,
  Peoples,
};
#ifndef __cplusplus
typedef uint8_t J_Tag;
#endif // __cplusplus

struct Hi_Body {
  int16_t _0;
};

struct Peoples_Body {
  uint8_t x;
  int16_t y;
};

struct J {
  J_Tag tag;
  union {
    struct Hi_Body hi;
    struct Peoples_Body peoples;
  };
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct A a, struct B b, C c, struct D d, union F f, struct H h, struct J j);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
