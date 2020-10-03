#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Opaque Opaque;

typedef struct Foo {
  const Opaque *x;
  Opaque *y;
  void (*z)(void);
} Foo;

typedef union Bar {
  const Opaque *x;
  Opaque *y;
  void (*z)(void);
} Bar;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const Opaque *a, Opaque *b, Foo c, Bar d);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
