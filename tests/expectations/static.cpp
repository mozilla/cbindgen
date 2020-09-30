#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Bar;

struct Foo {

};

extern "C" {

extern const Bar BAR;

extern Foo FOO;

extern const int32_t NUMBER;

void root();

} // extern "C"
