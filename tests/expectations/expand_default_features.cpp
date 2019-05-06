#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

struct Foo {

};

extern "C" {

void extra_debug_fn();

void root(Foo a);

} // extern "C"
