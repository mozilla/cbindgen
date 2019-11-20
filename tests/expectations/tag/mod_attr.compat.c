#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <uchar.h>

#if defined(BAR)
#define BAR 2
#endif

#if defined(FOO)
#define FOO 1
#endif

#if defined(BAR)
struct Bar {

};
#endif

#if defined(FOO)
struct Foo {

};
#endif

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#if defined(BAR)
void bar(const struct Bar *bar);
#endif

#if defined(FOO)
void foo(const struct Foo *foo);
#endif

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
