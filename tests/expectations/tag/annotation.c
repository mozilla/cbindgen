#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <uchar.h>

enum C {
  X = 2,
  Y,
};
typedef uint32_t C;

struct A {
  int32_t m0;
};

struct B {
  int32_t x;
  float y;
};

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

enum H_Tag {
  Hello,
  There,
  Everyone,
};
typedef uint8_t H_Tag;

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

void root(struct A x, struct B y, C z, union F f, struct H h);
