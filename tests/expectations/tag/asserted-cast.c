#define MY_ASSERT(...) do { } while (0)
#define MY_ATTRS __attribute((noinline))


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct I;

enum H_Tag {
  H_Foo,
  H_Bar,
  H_Baz,
};
typedef uint8_t H_Tag;

struct H_Foo_Body {
  int16_t _0;
};

struct H_Bar_Body {
  uint8_t x;
  int16_t y;
};

struct H {
  H_Tag tag;
  union {
    struct H_Foo_Body foo;
    struct H_Bar_Body bar;
  };
};

enum J_Tag {
  J_Foo,
  J_Bar,
  J_Baz,
};
typedef uint8_t J_Tag;

struct J_Foo_Body {
  int16_t _0;
};

struct J_Bar_Body {
  uint8_t x;
  int16_t y;
};

struct J {
  J_Tag tag;
  union {
    struct J_Foo_Body foo;
    struct J_Bar_Body bar;
  };
};

enum K_Tag {
  K_Foo,
  K_Bar,
  K_Baz,
};
typedef uint8_t K_Tag;

struct K_Foo_Body {
  K_Tag tag;
  int16_t _0;
};

struct K_Bar_Body {
  K_Tag tag;
  uint8_t x;
  int16_t y;
};

union K {
  K_Tag tag;
  struct K_Foo_Body foo;
  struct K_Bar_Body bar;
};

void foo(struct H h, struct I i, struct J j, union K k);
