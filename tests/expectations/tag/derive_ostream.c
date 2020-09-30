#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum C {
  X = 2,
  Y,
};
typedef uint32_t C;

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
  F_Tag tag;
  struct Foo_Body foo;
  struct Bar_Body bar;
};

void root(struct A a, struct B b, C c, struct D d, union F f);
