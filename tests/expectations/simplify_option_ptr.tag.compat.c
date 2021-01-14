#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Opaque;

struct Option_____Opaque;

struct Option_______c_void;

struct Foo {
  const struct Opaque *x;
  struct Opaque *y;
  void (*z)(void);
  struct Option_______c_void *zz;
};

union Bar {
  const struct Opaque *x;
  struct Opaque *y;
  void (*z)(void);
  struct Option_______c_void *zz;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const struct Opaque *a,
          struct Opaque *b,
          struct Foo c,
          union Bar d,
          struct Option_____Opaque *e);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
