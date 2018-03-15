#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

struct Normal {
  int32_t x;
  float y;
};

extern void bar(struct Normal a);

extern int32_t foo(void);
