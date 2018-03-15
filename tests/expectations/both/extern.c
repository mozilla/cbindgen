#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct Normal {
  int32_t x;
  float y;
} Normal;

extern void bar(Normal a);

extern int32_t foo(void);
