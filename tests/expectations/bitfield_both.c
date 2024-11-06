#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct HasBitfields;

typedef struct HasBitfields {
  uint64_t foo: 8;
  uint64_t bar: 56;
} HasBitfields;

void root(const struct HasBitfields*);
