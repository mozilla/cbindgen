#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

using DoFn = int32_t(*)(int32_t x, int32_t y);

struct StructWithOptionalFunctionPointer {
  DoFn func;
  int32_t (*maybe_func)(int32_t x, int32_t y);
};

template<typename T>
using NonNullAlias = T*;

struct StructWithOptionalNonNullPointer {
  NonNullAlias<uint32_t> data;
  uint32_t *maybe_data;
};

extern "C" {

void root(StructWithOptionalFunctionPointer swofp, StructWithOptionalNonNullPointer swonnp);

} // extern "C"
