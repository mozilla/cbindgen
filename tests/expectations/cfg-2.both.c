#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#if defined(NOT_DEFINED)
#define DEFAULT_X 8
#endif

#if defined(DEFINED)
#define DEFAULT_X 42
#endif

#if (defined(NOT_DEFINED) || defined(DEFINED))
typedef struct Foo {
  int32_t x;
} Foo;
#endif

#if defined(NOT_DEFINED)
typedef struct Bar {
  Foo y;
} Bar;
#endif

#if defined(DEFINED)
typedef struct Bar {
  Foo z;
} Bar;
#endif

typedef struct Root {
  Bar w;
} Root;

void root(Root a);
