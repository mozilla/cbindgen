#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct A {
  const int32_t *data;
} A;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(A _a);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
