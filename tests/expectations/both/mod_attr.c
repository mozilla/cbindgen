#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#if defined(BAR)
#define BAR 2
#endif

#if defined(FOO)
#define FOO 1
#endif

#if defined(BAR)
typedef struct Bar {

} Bar;
#endif

#if defined(FOO)
typedef struct Foo {

} Foo;
#endif

#if defined(BAR)
void bar(const Bar *bar);
#endif

#if defined(FOO)
void foo(const Foo *foo);
#endif
