#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct PREFIXBar {
  int32_t a;
} PREFIXBar;

typedef struct PREFIXFoo {
  int32_t a;
  uint32_t b;
  PREFIXBar bar;
} PREFIXFoo;

#define PREFIXVAL (PREFIXFoo){ .a = 42, .b = 1337, .bar = (PREFIXBar){ .a = 323 } }

void root(PREFIXFoo x);
