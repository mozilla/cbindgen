#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <uchar.h>

#define FOO 10

#define ZOM 3.14

typedef struct {
  int32_t x[FOO];
} Foo;

void root(Foo x);
