#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<uint32_t V>
struct TakeUntil {
  const uint8_t *start;
  uintptr_t len;
  uintptr_t point;
};

/// Dummy struct emitted by cbindgen to avoid compiler warnings/errors about
/// return type C linkage for template types returned by value from functions
struct __cbindgen_return_value_monomorphs {
  TakeUntil<0> field0;
};

extern "C" {

TakeUntil<0> until_nul(const uint8_t *start, uintptr_t len);

}  // extern "C"
