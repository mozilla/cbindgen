#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct ExtType {
  uint32_t data;
} ExtType;

void consume_ext(ExtType _ext);
