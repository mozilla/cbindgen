#include <cstdint>
#include <cstdlib>

struct Normal {
  int32_t x;
  float y;
};

struct TupleRenamed {
  int32_t m0;
  float m1;
};

struct TupleNamed {
  int32_t x;
  float y;
};

extern "C" {

void root(Normal b, TupleRenamed d, TupleNamed e);

} // extern "C"
