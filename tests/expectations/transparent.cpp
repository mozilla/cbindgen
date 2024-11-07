#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct DummyStruct;

struct EnumWithAssociatedConstantInImpl;

struct StructWithAssociatedConstantInImpl;

using TransparentComplexWrappingStructTuple = DummyStruct;

using TransparentPrimitiveWrappingStructTuple = uint32_t;

using TransparentComplexWrappingStructure = DummyStruct;

using TransparentPrimitiveWrappingStructure = uint32_t;

template<typename T>
using TransparentComplexWrapper = DummyStruct;

template<typename T>
using TransparentPrimitiveWrapper = uint32_t;

using TransparentPrimitiveWithAssociatedConstants = uint32_t;
constexpr static const TransparentPrimitiveWithAssociatedConstants TransparentPrimitiveWithAssociatedConstants_ZERO = 0;
constexpr static const TransparentPrimitiveWithAssociatedConstants TransparentPrimitiveWithAssociatedConstants_ONE = 1;

struct TransparentEmptyStructure {

};

using TransparentPointerWrappingStructure = const uint32_t*;

using TransparentIntStruct = int32_t;

using TransparentComplexStruct = DummyStruct;

using TransparentTransparentStruct = TransparentPrimitiveWrappingStructure;

using TransparentNonNullStruct = uint32_t*;

using TransparentOptionNonNullStruct = uint32_t*;

constexpr static const TransparentPrimitiveWrappingStructure StructWithAssociatedConstantInImpl_STRUCT_TEN = 10;

constexpr static const TransparentPrimitiveWrappingStructure EnumWithAssociatedConstantInImpl_ENUM_TEN = 10;

extern "C" {

void root(TransparentComplexWrappingStructTuple a,
          TransparentPrimitiveWrappingStructTuple b,
          TransparentComplexWrappingStructure c,
          TransparentPrimitiveWrappingStructure d,
          TransparentComplexWrapper<int32_t> e,
          TransparentPrimitiveWrapper<int32_t> f,
          TransparentPrimitiveWithAssociatedConstants g,
          TransparentEmptyStructure h,
          TransparentPointerWrappingStructure i,
          StructWithAssociatedConstantInImpl j,
          EnumWithAssociatedConstantInImpl k);

void erased_root(uint32_t *a,
                 uint32_t *b,
                 TransparentPrimitiveWrappingStructure c,
                 uint32_t *d,
                 TransparentIntStruct e,
                 int32_t f,
                 DummyStruct g,
                 uint32_t *h,
                 int32_t i,
                 TransparentIntStruct j,
                 TransparentComplexStruct k,
                 TransparentTransparentStruct l,
                 TransparentNonNullStruct m,
                 TransparentOptionNonNullStruct n);

}  // extern "C"
