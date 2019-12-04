#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Constants shared by multiple CSS Box Alignment properties
 *
 * These constants match Gecko's `NS_STYLE_ALIGN_*` constants.
 */
typedef struct StyleAlignFlags {
  uint8_t bits;
} StyleAlignFlags;
#define StyleAlignFlags_AUTO (StyleAlignFlags){ .bits = 0 }
#define StyleAlignFlags_NORMAL (StyleAlignFlags){ .bits = 1 }
#define StyleAlignFlags_START (StyleAlignFlags){ .bits = (1 << 1) }
#define StyleAlignFlags_END (StyleAlignFlags){ .bits = (1 << 2) }
#define StyleAlignFlags_FLEX_START (StyleAlignFlags){ .bits = (1 << 3) }

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(StyleAlignFlags flags);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
