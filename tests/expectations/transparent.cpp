#include <cstdint>
#include <cstdlib>

struct DummyStruct;

using TransparentComplexWrappingStructTuple = DummyStruct;

using TransparentPrimitiveWrappingStructTuple = uint32_t;

using TransparentComplexWrappingStructure = DummyStruct;

using TransparentPrimitiveWrappingStructure = uint32_t;

template<typename T>
using TransparentComplexWrapper = DummyStruct;

template<typename T>
using TransparentPrimitiveWrapper = uint32_t;

extern "C" {

void root(TransparentComplexWrappingStructTuple a,
          TransparentPrimitiveWrappingStructTuple b,
          TransparentComplexWrappingStructure c,
          TransparentPrimitiveWrappingStructure d,
          TransparentComplexWrapper<int32_t> e,
          TransparentPrimitiveWrapper<int32_t> f);

} // extern "C"
