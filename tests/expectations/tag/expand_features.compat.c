#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Foo {

};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void cbindgen(void);

void extra_debug_fn(void);

void root(struct Foo a);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
