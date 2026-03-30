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
  struct B Things;
} D;

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

typedef struct Bar_Body {
  F_Tag tag;
  uint8_t x;
  int16_t y;
} Bar_Body;

typedef union F {
  F_Tag tag;
  struct {
    F_Tag foo_tag;
    int16_t foo;
  };
  Bar_Body bar;
} F;

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

typedef struct There_Body {
  uint8_t x;
  int16_t y;
} There_Body;

typedef struct H {
  H_Tag tag;
  union {
    struct {
      int16_t hello;
    };
    There_Body there;
  };
} H;

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

void root(struct A a, struct B b, C c, struct D d, union F f, struct H h, struct I i);
