#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * A function without namespace annotation - uses global namespace
 */
void global_function(void);

/**
 * A function with a single namespace
 */
void ffi_function(void);

/**
 * A function with nested namespaces using :: separator
 */
void nested_function(const char *a);

/**
 * Another function with the same namespace to test grouping
 */
void another_nested_function(void);

/**
 * A function with a different nested namespace
 */
void other_namespace_function(void);
