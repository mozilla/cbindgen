#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Foo_Bar {
  const int32_t *something;
} Foo_Bar;

enum Bar_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  Min,
  Max,
  Other,
};
#if __STDC_VERSION__ >= 202311L
typedef enum Bar_Tag Bar_Tag;
#else
typedef uint8_t Bar_Tag;
#endif // __STDC_VERSION__ >= 202311L

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

void root(union Bar b);
