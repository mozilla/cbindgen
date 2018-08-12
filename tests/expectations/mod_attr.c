#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

#if defined(BAR)
#define BAR 2
#endif

#if defined(FOO)
#define FOO 1
#endif

#if defined(BAR)
typedef struct {

} Bar;
#endif

#if defined(FOO)
typedef struct {

} Foo;
#endif

#if defined(BAR)
void bar(const Bar *bar);
#endif

#if defined(FOO)
void foo(const Foo *foo);
#endif
