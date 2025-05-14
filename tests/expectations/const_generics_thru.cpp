#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<uintptr_t N>
struct Inner {
  uint8_t bytes[N];
};

template<uintptr_t N>
struct Outer {
  Inner<N> inner;
};

/// Dummy struct emitted by cbindgen to avoid compiler warnings/errors about
/// return type C linkage for template types returned by value from functions
struct __cbindgen_return_value_monomorphs {
  Outer<1> field0;
  Outer<2> field1;
};

extern "C" {

Outer<1> one();

Outer<2> two();

}  // extern "C"
