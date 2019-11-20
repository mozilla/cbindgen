#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <uchar.h>

struct Bar;

struct Foo {

};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern const struct Bar BAR;

extern struct Foo FOO;

extern const int32_t NUMBER;

void root(void);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
