#include <cstdint>
#include <cstdlib>

extern "C" {

struct IntFoo_i32 {
  int32_t x;
  int32_t y;
};

void root(IntFoo_i32 a);

} // extern "C"

template<typename T>
struct IntFoo;

template<>
struct IntFoo<int32_t> : public IntFoo_i32 {

};
