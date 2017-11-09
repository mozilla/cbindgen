#include <cstdint>
#include <cstdlib>

extern "C" {

static const char* BAR = u8"hello world";

static const int32_t FOO = 10;

static const float ZOM = 3.14;

struct Foo {
  int32_t x[FOO];
};

void root(Foo x);

} // extern "C"
