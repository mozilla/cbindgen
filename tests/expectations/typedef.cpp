#include <cstdint>
#include <cstdlib>

template<typename T>
struct IntFoo {
  int32_t x;
  T y;
};

extern "C" {

void root(IntFoo<int32_t> a);

} // extern "C"
