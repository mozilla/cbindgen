#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Opaque;

struct Foo {
  const struct Opaque *x;
  struct Opaque *y;
  void (*z)(void);
};

union Bar {
  const struct Opaque *x;
  struct Opaque *y;
  void (*z)(void);
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const struct Opaque *a, struct Opaque *b, struct Foo c, union Bar d);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
