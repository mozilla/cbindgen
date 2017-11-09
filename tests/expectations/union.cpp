#include <cstdint>
#include <cstdlib>

extern "C" {

struct Opaque;

union Normal {
  int32_t x;
  float y;
};

union NormalWithZST {
  int32_t x;
  float y;
};

void root(Opaque *a, Normal b, NormalWithZST c);

} // extern "C"
