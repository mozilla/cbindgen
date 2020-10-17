#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Foo_Bar {
  const int32_t *something;
};

enum Bar_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  Min,
  Max,
  Other,
};
#ifndef __cplusplus
typedef uint8_t Bar_Tag;
#endif // __cplusplus

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

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(union Bar b);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
