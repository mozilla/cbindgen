#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum C
#if __STDC_VERSION__ >= 202311L
  : uint32_t
#endif // __STDC_VERSION__ >= 202311L
 {
  X = 2,
  Y,
};
#if __STDC_VERSION__ >= 202311L
typedef enum C C;
#else
typedef uint32_t C;
#endif // __STDC_VERSION__ >= 202311L

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
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  Foo,
  Bar,
  Baz,
};
#if __STDC_VERSION__ >= 202311L
typedef enum F_Tag F_Tag;
#else
typedef uint8_t F_Tag;
#endif // __STDC_VERSION__ >= 202311L

struct Bar_Body {
  F_Tag tag;
  uint8_t x;
  int16_t y;
};

union F {
  F_Tag tag;
  struct {
    F_Tag foo_tag;
    int16_t foo;
  };
  struct Bar_Body bar;
};

enum H_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  Hello,
  There,
  Everyone,
};
#if __STDC_VERSION__ >= 202311L
typedef enum H_Tag H_Tag;
#else
typedef uint8_t H_Tag;
#endif // __STDC_VERSION__ >= 202311L

struct There_Body {
  uint8_t x;
  int16_t y;
};

struct H {
  H_Tag tag;
  union {
    struct {
      int16_t hello;
    };
    struct There_Body there;
  };
};

enum I_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  ThereAgain,
  SomethingElse,
};
#if __STDC_VERSION__ >= 202311L
typedef enum I_Tag I_Tag;
#else
typedef uint8_t I_Tag;
#endif // __STDC_VERSION__ >= 202311L

struct ThereAgain_Body {
  uint8_t x;
  int16_t y;
};

struct I {
  I_Tag tag;
  union {
    struct ThereAgain_Body there_again;
  };
};

void root(struct A a, struct B b, C c, struct D d, union F f, struct H h, struct I i);
