#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef int32_t (*DoFn)(int32_t x, int32_t y);

typedef struct {
  DoFn func;
  DoFn maybe_func;
} StructWithOptionalFunctionPointer;

typedef uint32_t *NonNullAlias_u32;

typedef struct {
  NonNullAlias_u32 data;
  NonNullAlias_u32 maybe_data;
} StructWithOptionalNonNullPointer;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(StructWithOptionalFunctionPointer swofp, StructWithOptionalNonNullPointer swonnp);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
