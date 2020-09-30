#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

#if defined(BAR)
static const int32_t BAR = 2;
#endif

#if defined(FOO)
static const int32_t FOO = 1;
#endif

#if defined(BAR)
struct Bar {

};
#endif

#if defined(FOO)
struct Foo {

};
#endif

extern "C" {

#if defined(BAR)
void bar(const Bar *bar);
#endif

#if defined(FOO)
void foo(const Foo *foo);
#endif

} // extern "C"
