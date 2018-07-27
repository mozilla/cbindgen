#include <cstdint>
#include <cstdlib>

struct Foo {

};

extern "C" {

void cbindgen();

void extra_debug_fn();

void root(Foo a);

} // extern "C"
