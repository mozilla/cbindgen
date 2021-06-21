#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef int32_t (*DoFn)(int32_t x, int32_t y);

typedef struct StructWithOptionalFunctionPointer {
  DoFn func;
  int32_t (*maybe_func)(int32_t x, int32_t y);
} StructWithOptionalFunctionPointer;

typedef uint32_t *NonNullAlias_u32;

typedef struct StructWithOptionalNonNullPointer {
  NonNullAlias_u32 data;
  uint32_t *maybe_data;
} StructWithOptionalNonNullPointer;

void root(struct StructWithOptionalFunctionPointer swofp,
          struct StructWithOptionalNonNullPointer swonnp);
