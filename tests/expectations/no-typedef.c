#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

struct simple {
  uint64_t len;
};

const struct simple *simple(const struct simple *simple);
