#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

struct Bar;

struct Foo {
  int32_t a;
  uint32_t b;
};
static const Foo Foo_FOO = { /* .a = */ 42, /* .b = */ 47 };
static const Foo Foo_FOO2 = { /* .a = */ 42, /* .b = */ 47 };
static const Foo Foo_FOO3 = { /* .a = */ 42, /* .b = */ 47 };


static const Foo BAR = { /* .a = */ 42, /* .b = */ 1337 };



extern "C" {

void root(Foo x, Bar bar);

} // extern "C"
