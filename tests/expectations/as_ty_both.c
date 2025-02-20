#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define SIZE 4

typedef struct WithoutAs {
  uint32_t items[SIZE];
} WithoutAs;

typedef struct WithAs {
  uint32_t items[SIZE];
} WithAs;

void some_fn(struct WithoutAs a, struct WithAs b);
