#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

#define EXPORT_ME_TOO 42

typedef struct {
  uint64_t val;
} ExportMe;

void export_me(ExportMe *val);
