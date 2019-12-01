#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <uchar.h>

struct Foo {
  char32_t a;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct Foo a);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
