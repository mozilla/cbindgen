#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  uint8_t *a;
} Foo_____u8;

typedef Foo_____u8 Boo;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(Boo x);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
