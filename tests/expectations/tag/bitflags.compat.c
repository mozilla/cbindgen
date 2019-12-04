#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Constants shared by multiple CSS Box Alignment properties
 *
 * These constants match Gecko's `NS_STYLE_ALIGN_*` constants.
 */
struct AlignFlags {
  uint8_t bits;
};
#define AlignFlags_AUTO (AlignFlags){ .bits = 0 }
#define AlignFlags_NORMAL (AlignFlags){ .bits = 1 }
#define AlignFlags_START (AlignFlags){ .bits = (1 << 1) }
#define AlignFlags_END (AlignFlags){ .bits = (1 << 2) }
#define AlignFlags_FLEX_START (AlignFlags){ .bits = (1 << 3) }

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct AlignFlags flags);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
