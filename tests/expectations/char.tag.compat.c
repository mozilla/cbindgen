#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Foo {
  uint32_t a;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct Foo a);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
