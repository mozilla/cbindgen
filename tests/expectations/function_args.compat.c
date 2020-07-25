#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void pointer_test(const uint64_t *a);

void print_from_rust(void);

void unnamed(const uint64_t*);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
