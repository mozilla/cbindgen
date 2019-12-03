#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  int32_t a;
} PREFIXBar;

typedef struct {
  int32_t a;
  uint32_t b;
  PREFIXBar bar;
} PREFIXFoo;

#define PREFIXVAL (PREFIXFoo){ .a = 42, .b = 1337, .bar = (PREFIXBar){ .a = 323 } }

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(PREFIXFoo x);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
