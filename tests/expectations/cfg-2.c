#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

#if (defined(NOT_DEFINED) || defined(DEFINED))
typedef struct {
  int32_t x;
} Foo;
#endif

#if defined(NOT_DEFINED)
typedef struct {
  Foo y;
} Bar;
#endif

#if defined(DEFINED)
typedef struct {
  Foo z;
} Bar;
#endif

typedef struct {
  Bar w;
} Root;

void root(Root a);
