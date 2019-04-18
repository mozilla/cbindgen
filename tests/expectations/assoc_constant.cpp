#include <cstdarg>
#include <cstdint>
#include <cstdlib>

struct Foo {

};
static const int32_t Foo_GA = 10;
static const wchar_t* Foo_BU = L"hello world";
static const float Foo_ZO = 3.14;

extern "C" {

void root(Foo x);

} // extern "C"
