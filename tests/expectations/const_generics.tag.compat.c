#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define SOME_NUMBER 20

struct StructWithConstGeneric_SOME_NUMBER;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const struct StructWithConstGeneric_SOME_NUMBER *a);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
