#include <cstdarg>
#include <cstdint>
#include <cstdlib>

namespace constants {

static const int32_t FOO = 10;

static const float ZOM = 3.14;

struct Foo {
  int32_t x[FOO];
};

extern "C" {

void root(Foo x);

} // extern "C"

} // namespace constants
