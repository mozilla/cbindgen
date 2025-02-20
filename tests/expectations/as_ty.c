#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define SIZE 4

typedef struct {
  uint32_t items[SIZE];
} WithoutAs;

typedef struct {
  uint32_t items[SIZE];
} WithAs;

void some_fn(WithoutAs a, WithAs b);
