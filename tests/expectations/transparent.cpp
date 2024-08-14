#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class TransparentEnumWithNoData {
  NoData,
};

struct DummyStruct;

struct EnumWithAssociatedConstantInImpl;

struct StructWithAssociatedConstantInImpl;

using TransparentComplexWrappingStructTuple = DummyStruct;

using TransparentPrimitiveWrappingStructTuple = uint32_t;

using TransparentComplexWrappingStruct = DummyStruct;

using TransparentPrimitiveWrappingStruct = uint32_t;

template<typename T>
using TransparentComplexWrapperStruct = DummyStruct;

template<typename T>
using TransparentPrimitiveWrapperStruct = uint32_t;

using TransparentPrimitiveStructWithAssociatedConstants = uint32_t;
constexpr static const TransparentPrimitiveStructWithAssociatedConstants TransparentPrimitiveStructWithAssociatedConstants_STRUCT_ZERO = 0;
constexpr static const TransparentPrimitiveStructWithAssociatedConstants TransparentPrimitiveStructWithAssociatedConstants_STRUCT_ONE = 1;

struct TransparentEmptyStruct {

};

using TransparentPointerWrappingStruct = const uint32_t*;

using TransparentIntStruct = int32_t;

using TransparentComplexStruct = DummyStruct;

using TransparentTransparentStruct = TransparentPrimitiveWrappingStruct;

using TransparentNonNullStruct = uint32_t*;

using TransparentOptionNonNullStruct = uint32_t*;

using TransparentComplexWrappingEnumTuple = DummyStruct;

using TransparentPrimitiveWrappingEnumTuple = uint32_t;

using TransparentComplexWrappingEnum = DummyStruct;

using TransparentPrimitiveWrappingEnum = uint32_t;

template<typename T>
using TransparentComplexWrapperEnum = DummyStruct;

template<typename T>
using TransparentPrimitiveWrapperEnum = uint32_t;

struct TransparentEnumWithEmptyData {
  enum class Tag {
    EmptyData,
  };

  struct EmptyData_Body {

  };

  Tag tag;
  union {
    EmptyData_Body empty_data;
  };
};

using TransparentPrimitiveEnumWithAssociatedConstants = uint32_t;

using TransparentPointerWrappingEnum = const uint32_t*;

using TransparentIntEnum = int32_t;

using TransparentComplexEnum = DummyStruct;

using TransparentTransparentEnum = TransparentPrimitiveWrappingEnum;

using TransparentNonNullEnum = uint32_t*;

using TransparentOptionNonNullEnum = uint32_t*;

constexpr static const TransparentPrimitiveWrappingStruct StructWithAssociatedConstantInImpl_STRUCT_TEN = 10;





constexpr static const TransparentPrimitiveWrappingStruct EnumWithAssociatedConstantInImpl_ENUM_TEN = 10;

extern "C" {

void struct_root(TransparentComplexWrappingStructTuple a,
                 TransparentPrimitiveWrappingStructTuple b,
                 TransparentComplexWrappingStruct c,
                 TransparentPrimitiveWrappingStruct d,
                 TransparentComplexWrapperStruct<int32_t> e,
                 TransparentPrimitiveWrapperStruct<int32_t> f,
                 TransparentPrimitiveStructWithAssociatedConstants g,
                 TransparentEmptyStruct h,
                 TransparentPointerWrappingStruct i,
                 StructWithAssociatedConstantInImpl j);

void erased_root(uint32_t *a,
                 uint32_t *b,
                 TransparentPrimitiveWrappingStruct c,
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

void enum_root(TransparentComplexWrappingEnumTuple a,
               TransparentPrimitiveWrappingEnumTuple b,
               TransparentComplexWrappingEnum c,
               TransparentPrimitiveWrappingEnum d,
               TransparentComplexWrapperEnum<int32_t> e,
               TransparentPrimitiveWrapperEnum<int32_t> f,
               TransparentEnumWithNoData g,
               TransparentEnumWithEmptyData h,
               TransparentPrimitiveEnumWithAssociatedConstants i,
               TransparentPointerWrappingEnum j,
               EnumWithAssociatedConstantInImpl k);

void erased_enum_root(uint32_t *a,
                      uint32_t *b,
                      TransparentPrimitiveWrappingEnum c,
                      uint32_t *d,
                      TransparentIntEnum e,
                      int32_t f,
                      DummyStruct g,
                      uint32_t *h,
                      int32_t i,
                      TransparentIntEnum j,
                      TransparentComplexEnum k,
                      TransparentTransparentEnum l,
                      TransparentNonNullEnum m,
                      TransparentOptionNonNullEnum n);

}  // extern "C"
