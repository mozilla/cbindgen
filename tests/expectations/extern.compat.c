#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  int32_t x;
  float y;
} Normal;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern int32_t foo(void);

extern void bar(Normal a);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
