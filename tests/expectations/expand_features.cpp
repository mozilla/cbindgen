#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Foo {

};

extern "C" {

void cbindgen();

void extra_debug_fn();

void root(Foo a);

} // extern "C"
