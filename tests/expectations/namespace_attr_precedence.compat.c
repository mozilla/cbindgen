#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
namespace global_ns {
#endif  // __cplusplus

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * A function without namespace annotation - should use global namespace
 */
void uses_global_namespace(void);

/**
 * A function with per-item namespace - should override global namespace
 */
void uses_item_namespace(const char *a);

/**
 * Another function without namespace annotation - should use global namespace
 */
void also_uses_global_namespace(void);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#ifdef __cplusplus
}  // namespace global_ns
#endif  // __cplusplus
