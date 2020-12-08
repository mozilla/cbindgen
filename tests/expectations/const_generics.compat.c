#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define SOME_NUMBER 20

typedef struct StructWithConstGeneric_SOME_NUMBER StructWithConstGeneric_SOME_NUMBER;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const StructWithConstGeneric_SOME_NUMBER *a);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
