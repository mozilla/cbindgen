#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Matches glob patterns such as `src/\*\/mod.rs`.
 *
 * A C comment like /\* this *\/ in the docs must not end the generated comment,
 * and neither must `**\/\*.rs`.
 */
void root(void);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
