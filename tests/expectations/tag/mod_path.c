#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

#define EXPORT_ME_TOO 42

struct ExportMe {
  uint64_t val;
};

void export_me(struct ExportMe *val);
