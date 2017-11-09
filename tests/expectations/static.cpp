#include <cstdint>
#include <cstdlib>

extern "C" {

struct Bar;

struct Foo {

};

extern const Bar BAR;

extern Foo FOO;

extern const int32_t NUMBER;

extern const char* STRING;

void root();

} // extern "C"
