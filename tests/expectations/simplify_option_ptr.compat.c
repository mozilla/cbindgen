#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Opaque Opaque;

typedef struct Option_____Opaque Option_____Opaque;

typedef struct Option_______c_void Option_______c_void;

typedef struct {
  const Opaque *x;
  Opaque *y;
  void (*z)(void);
  Option_______c_void *zz;
} Foo;

typedef union {
  const Opaque *x;
  Opaque *y;
  void (*z)(void);
  Option_______c_void *zz;
} Bar;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const Opaque *a, Opaque *b, Foo c, Bar d, Option_____Opaque *e);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
