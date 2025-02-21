#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Foo {

};
constexpr static const int32_t FOO_GA = 10;
constexpr static const float FOO_ZO = 3.14;

extern "C" {

void root(Foo x);

}  // extern "C"
