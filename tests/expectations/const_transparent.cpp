#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

constexpr static const int64_t TransparentEnum_ASSOC_ENUM_FOO = 8;

template<typename T>
using Wrapper = T;

using TransparentStruct = uint8_t;
constexpr static const int64_t TransparentStruct_ASSOC_STRUCT_FOO = 1;
constexpr static const TransparentStruct TransparentStruct_ASSOC_STRUCT_BAR = 2;
constexpr static const Wrapper<TransparentStruct> TransparentStruct_ASSOC_STRUCT_BAZ = 3;

using TransparentTupleStruct = uint8_t;

template<typename T>
using TransparentStructWithErasedField = T;

using TransparentEnum = uint8_t;

template<typename T>
using TransparentWrapperEnum = T;

constexpr static const TransparentStruct STRUCT_FOO = 4;

constexpr static const TransparentTupleStruct STRUCT_BAR = 5;

constexpr static const TransparentStruct STRUCT_BAZ = 6;

constexpr static const TransparentStructWithErasedField<TransparentStruct> COMPLEX = 7;








