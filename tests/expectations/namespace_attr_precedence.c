#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * A function without namespace attribute - should use global namespace
 */
void uses_global_namespace(void);

/**
 * A function with per-item namespace - should override global namespace
 */
void uses_item_namespace(const char *a);

/**
 * Another function without namespace attribute - should use global namespace
 */
void also_uses_global_namespace(void);
