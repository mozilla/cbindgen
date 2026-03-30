#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Foo_Bar {
  const int32_t *something;
} Foo_Bar;

enum Bar_Tag
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Min,
  Max,
  Other,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Bar_Tag Bar_Tag;
#else
typedef uint8_t Bar_Tag;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

typedef union Bar {
  Bar_Tag tag;
  struct {
    Bar_Tag min_tag;
    struct Foo_Bar min;
  };
  struct {
    Bar_Tag max_tag;
    struct Foo_Bar max;
  };
} Bar;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(union Bar b);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
