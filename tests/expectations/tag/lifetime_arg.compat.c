#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <uchar.h>

struct A {
  const int32_t *data;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct A _a);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
