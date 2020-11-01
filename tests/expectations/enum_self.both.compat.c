#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Foo_Bar {
  const int32_t *something;
} Foo_Bar;

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

typedef struct Min_Body {
  Bar_Tag tag;
  struct Foo_Bar _0;
} Min_Body;

typedef struct Max_Body {
  Bar_Tag tag;
  struct Foo_Bar _0;
} Max_Body;

typedef union Bar {
  Bar_Tag tag;
  Min_Body min;
  Max_Body max;
} Bar;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(union Bar b);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
