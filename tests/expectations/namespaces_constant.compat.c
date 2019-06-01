#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define FOO 10

#define ZOM 3.14

typedef struct {
  int32_t x[FOO];
} Foo;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(Foo x);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
