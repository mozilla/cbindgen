#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define SIZE 4

struct WithoutAs {
  uint32_t items[SIZE];
};

struct WithAs {
  uint32_t items[SIZE];
};

void some_fn(struct WithoutAs a, struct WithAs b);
