#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum TransparentEnumWithNoData {
  NoData,
} TransparentEnumWithNoData;

typedef struct DummyStruct DummyStruct;

typedef struct EnumWithAssociatedConstantInImpl EnumWithAssociatedConstantInImpl;

typedef struct StructWithAssociatedConstantInImpl StructWithAssociatedConstantInImpl;

typedef struct DummyStruct TransparentComplexWrappingStructTuple;

typedef uint32_t TransparentPrimitiveWrappingStructTuple;

typedef struct DummyStruct TransparentComplexWrappingStruct;

typedef uint32_t TransparentPrimitiveWrappingStruct;

typedef struct DummyStruct TransparentComplexWrapperStruct_i32;

typedef uint32_t TransparentPrimitiveWrapperStruct_i32;

typedef uint32_t TransparentPrimitiveStructWithAssociatedConstants;
#define TransparentPrimitiveStructWithAssociatedConstants_STRUCT_ZERO 0
#define TransparentPrimitiveStructWithAssociatedConstants_STRUCT_ONE 1

typedef struct TransparentEmptyStruct {

} TransparentEmptyStruct;

typedef const uint32_t *TransparentPointerWrappingStruct;

typedef int32_t TransparentIntStruct;

typedef struct DummyStruct TransparentComplexStruct;

typedef TransparentPrimitiveWrappingStruct TransparentTransparentStruct;

typedef uint32_t *TransparentNonNullStruct;

typedef uint32_t *TransparentOptionNonNullStruct;

typedef struct DummyStruct TransparentComplexWrappingEnumTuple;

typedef uint32_t TransparentPrimitiveWrappingEnumTuple;

typedef struct DummyStruct TransparentComplexWrappingEnum;

typedef uint32_t TransparentPrimitiveWrappingEnum;

typedef struct DummyStruct TransparentComplexWrapperEnum_i32;

typedef uint32_t TransparentPrimitiveWrapperEnum_i32;

typedef enum TransparentEnumWithEmptyData_Tag {
  EmptyData,
} TransparentEnumWithEmptyData_Tag;

typedef struct EmptyData_Body {

} EmptyData_Body;

typedef struct TransparentEnumWithEmptyData {
  TransparentEnumWithEmptyData_Tag tag;
  union {
    EmptyData_Body empty_data;
  };
} TransparentEnumWithEmptyData;

typedef uint32_t TransparentPrimitiveEnumWithAssociatedConstants;

typedef const uint32_t *TransparentPointerWrappingEnum;

typedef int32_t TransparentIntEnum;

typedef struct DummyStruct TransparentComplexEnum;

typedef TransparentPrimitiveWrappingEnum TransparentTransparentEnum;

typedef uint32_t *TransparentNonNullEnum;

typedef uint32_t *TransparentOptionNonNullEnum;

#define StructWithAssociatedConstantInImpl_STRUCT_TEN 10





#define EnumWithAssociatedConstantInImpl_ENUM_TEN 10

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void struct_root(TransparentComplexWrappingStructTuple a,
                 TransparentPrimitiveWrappingStructTuple b,
                 TransparentComplexWrappingStruct c,
                 TransparentPrimitiveWrappingStruct d,
                 TransparentComplexWrapperStruct_i32 e,
                 TransparentPrimitiveWrapperStruct_i32 f,
                 TransparentPrimitiveStructWithAssociatedConstants g,
                 struct TransparentEmptyStruct h,
                 TransparentPointerWrappingStruct i,
                 struct StructWithAssociatedConstantInImpl j);

void erased_root(uint32_t *a,
                 uint32_t *b,
                 TransparentPrimitiveWrappingStruct c,
                 uint32_t *d,
                 TransparentIntStruct e,
                 int32_t f,
                 struct DummyStruct g,
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
               TransparentComplexWrapperEnum_i32 e,
               TransparentPrimitiveWrapperEnum_i32 f,
               enum TransparentEnumWithNoData g,
               struct TransparentEnumWithEmptyData h,
               TransparentPrimitiveEnumWithAssociatedConstants i,
               TransparentPointerWrappingEnum j,
               struct EnumWithAssociatedConstantInImpl k);

void erased_enum_root(uint32_t *a,
                      uint32_t *b,
                      TransparentPrimitiveWrappingEnum c,
                      uint32_t *d,
                      TransparentIntEnum e,
                      int32_t f,
                      struct DummyStruct g,
                      uint32_t *h,
                      int32_t i,
                      TransparentIntEnum j,
                      TransparentComplexEnum k,
                      TransparentTransparentEnum l,
                      TransparentNonNullEnum m,
                      TransparentOptionNonNullEnum n);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
