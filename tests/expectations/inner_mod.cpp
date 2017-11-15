#include <cstdint>
#include <cstdlib>

struct Foo {
  float x;
};

extern "C" {

void root(Foo a);

} // extern "C"
