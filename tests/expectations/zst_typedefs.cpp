#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T>
using C = void(*)(T, uint8_t);

template<typename T>
using D = const T(*)[2];

template<typename T>
struct Wrapper {
  C<T> c;
  D<T> d;
};

extern "C" {

void use_typedefs_ptr(const Wrapper<void> *w);

}  // extern "C"
