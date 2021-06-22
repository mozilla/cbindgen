#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef int32_t (*DoFn)(int32_t x, int32_t y);

struct StructWithOptionalFunctionPointer {
  DoFn func;
  DoFn maybe_func;
};

typedef uint32_t *NonNullAlias_u32;

struct StructWithOptionalNonNullPointer {
  NonNullAlias_u32 data;
  NonNullAlias_u32 maybe_data;
};

void root(struct StructWithOptionalFunctionPointer swofp,
          struct StructWithOptionalNonNullPointer swonnp);
