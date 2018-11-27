#include <cstdarg>
#include <cstdint>
#include <cstdlib>

struct Bar;

struct Foo {

};

extern "C" {

extern const Bar BAR;

extern Foo FOO;

extern const int32_t NUMBER;

void root();

} // extern "C"
