#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T>
struct MyStruct {
  uint32_t int_field;
  T generic_field;
};

extern "C" {

MyStruct<void> my_test();

} // extern "C"
