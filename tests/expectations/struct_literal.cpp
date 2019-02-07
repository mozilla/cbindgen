#include <cstdarg>
#include <cstdint>
#include <cstdlib>

struct Foo {
  int32_t a;
  uint32_t b;
};

static const Foo BAR = (Foo){ .a = 42, .b = 1337 };

static const Foo Foo_FOO = (Foo){ .a = 42, .b = 47 };

extern "C" {

void root(Foo x);

} // extern "C"
