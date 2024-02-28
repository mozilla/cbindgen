#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

/**
 * Outer docs should be exported.
 */
typedef struct {
  Inner *inner;
} Outer;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const Outer *a);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
