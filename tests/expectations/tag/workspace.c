#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

struct ExtType {
  uint32_t data;
};

void consume_ext(struct ExtType _ext);
