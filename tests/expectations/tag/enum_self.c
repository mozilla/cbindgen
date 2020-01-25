#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Foo_Bar {
  const int32_t *something;
};

enum Bar_Tag {
  Min,
  Max,
  Other,
};
typedef uint8_t Bar_Tag;

struct Min_Body {
  Bar_Tag tag;
  struct Foo_Bar _0;
};

struct Max_Body {
  Bar_Tag tag;
  struct Foo_Bar _0;
};

union Bar {
  Bar_Tag tag;
  struct Min_Body min;
  struct Max_Body max;
};

void root(union Bar b);
