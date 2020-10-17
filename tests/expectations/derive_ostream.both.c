#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum C {
  X = 2,
  Y,
};
typedef uint32_t C;

typedef struct A {
  int32_t _0;
} A;

typedef struct B {
  int32_t x;
  float y;
} B;

typedef struct D {
  uint8_t List;
  uintptr_t Of;
  B Things;
} D;

enum F_Tag {
  Foo,
  Bar,
  Baz,
};
typedef uint8_t F_Tag;

typedef struct Foo_Body {
  F_Tag tag;
  int16_t _0;
} Foo_Body;

typedef struct Bar_Body {
  F_Tag tag;
  uint8_t x;
  int16_t y;
} Bar_Body;

typedef union F {
  F_Tag tag;
  Foo_Body foo;
  Bar_Body bar;
} F;

enum H_Tag {
  Hello,
  There,
  Everyone,
};
typedef uint8_t H_Tag;

typedef struct Hello_Body {
  int16_t _0;
} Hello_Body;

typedef struct There_Body {
  uint8_t x;
  int16_t y;
} There_Body;

typedef struct H {
  H_Tag tag;
  union {
    Hello_Body hello;
    There_Body there;
  };
} H;

enum I_Tag {
  ThereAgain,
  SomethingElse,
};
typedef uint8_t I_Tag;

typedef struct ThereAgain_Body {
  uint8_t x;
  int16_t y;
} ThereAgain_Body;

typedef struct I {
  I_Tag tag;
  union {
    ThereAgain_Body there_again;
  };
} I;

void root(A a, B b, C c, D d, F f, H h, I i);
