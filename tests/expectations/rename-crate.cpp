#include <cstdarg>
#include <cstdint>
#include <cstdlib>

struct Foo {
  int32_t x;
};

extern "C" {

void root(Foo a);

} // extern "C"
