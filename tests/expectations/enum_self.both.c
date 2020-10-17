#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Foo_Bar {
  const int32_t *something;
} Foo_Bar;

enum Bar_Tag {
  Min,
  Max,
  Other,
};
typedef uint8_t Bar_Tag;

typedef struct Min_Body {
  Bar_Tag tag;
  Foo_Bar _0;
} Min_Body;

typedef struct Max_Body {
  Bar_Tag tag;
  Foo_Bar _0;
} Max_Body;

typedef union Bar {
  Bar_Tag tag;
  Min_Body min;
  Max_Body max;
} Bar;

void root(Bar b);
