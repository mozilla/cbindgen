#define MY_ASSERT(...) do { } while (0)


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct I I;

enum H_Tag {
  H_Foo,
  H_Bar,
  H_Baz,
};
typedef uint8_t H_Tag;

typedef struct H_Foo_Body {
  int16_t _0;
} H_Foo_Body;

typedef struct H_Bar_Body {
  uint8_t x;
  int16_t y;
} H_Bar_Body;

typedef struct H {
  H_Tag tag;
  union {
    H_Foo_Body foo;
    H_Bar_Body bar;
  };
} H;

enum J_Tag {
  J_Foo,
  J_Bar,
  J_Baz,
};
typedef uint8_t J_Tag;

typedef struct J_Foo_Body {
  int16_t _0;
} J_Foo_Body;

typedef struct J_Bar_Body {
  uint8_t x;
  int16_t y;
} J_Bar_Body;

typedef struct J {
  J_Tag tag;
  union {
    J_Foo_Body foo;
    J_Bar_Body bar;
  };
} J;

enum K_Tag {
  K_Foo,
  K_Bar,
  K_Baz,
};
typedef uint8_t K_Tag;

typedef struct K_Foo_Body {
  K_Tag tag;
  int16_t _0;
} K_Foo_Body;

typedef struct K_Bar_Body {
  K_Tag tag;
  uint8_t x;
  int16_t y;
} K_Bar_Body;

typedef union K {
  K_Tag tag;
  K_Foo_Body foo;
  K_Bar_Body bar;
} K;

void foo(H h, I i, J j, K k);
