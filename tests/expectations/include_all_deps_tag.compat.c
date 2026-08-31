#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Reachable from an exported function, so it is emitted even though its crate
 * does not contribute top-level items.
 */
struct UsedDepStruct {
  uint32_t x;
};

/**
 * Not used by any exported item, but declared by the binding crate.
 */
struct UnusedLocalStruct {
  uint32_t z;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

uint32_t get_x(const struct UsedDepStruct *used);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
