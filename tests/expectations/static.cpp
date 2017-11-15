#include <cstdint>
#include <cstdlib>

struct Bar;

struct Foo {

};

extern "C" {

extern const Bar BAR;

extern Foo FOO;

extern const int32_t NUMBER;

extern const char* STRING;

void root();

} // extern "C"
