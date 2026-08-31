#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Reachable from an exported function, so it is emitted even though its crate
 * does not contribute top-level items.
 */
typedef struct {
  uint32_t x;
} UsedDepStruct;

/**
 * Not used by any exported item, but declared by the binding crate.
 */
typedef struct {
  uint32_t z;
} UnusedLocalStruct;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

uint32_t get_x(const UsedDepStruct *used);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
