#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct FooU8 {
  uint8_t a;
} FooU8;

typedef FooU8 Boo;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(Boo x);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
