#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct DummyStruct DummyStruct;

typedef struct EnumWithAssociatedConstantInImpl EnumWithAssociatedConstantInImpl;

typedef struct StructWithAssociatedConstantInImpl StructWithAssociatedConstantInImpl;

typedef struct DummyStruct TransparentComplexWrappingStructTuple;

typedef uint32_t TransparentPrimitiveWrappingStructTuple;

typedef struct DummyStruct TransparentComplexWrappingStructure;

typedef uint32_t TransparentPrimitiveWrappingStructure;

typedef struct DummyStruct TransparentComplexWrapper_i32;

typedef uint32_t TransparentPrimitiveWrapper_i32;

typedef uint32_t TransparentPrimitiveWithAssociatedConstants;
#define TransparentPrimitiveWithAssociatedConstants_ZERO 0
#define TransparentPrimitiveWithAssociatedConstants_ONE 1

typedef struct TransparentEmptyStructure {

} TransparentEmptyStructure;

typedef const uint32_t *TransparentPointerWrappingStructure;

typedef int32_t TransparentIntStruct;

typedef struct DummyStruct TransparentComplexStruct;

typedef TransparentPrimitiveWrappingStructure TransparentTransparentStruct;

typedef uint32_t *TransparentNonNullStruct;

typedef uint32_t *TransparentOptionNonNullStruct;

#define StructWithAssociatedConstantInImpl_STRUCT_TEN 10

#define EnumWithAssociatedConstantInImpl_ENUM_TEN 10

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(TransparentComplexWrappingStructTuple a,
          TransparentPrimitiveWrappingStructTuple b,
          TransparentComplexWrappingStructure c,
          TransparentPrimitiveWrappingStructure d,
          TransparentComplexWrapper_i32 e,
          TransparentPrimitiveWrapper_i32 f,
          TransparentPrimitiveWithAssociatedConstants g,
          struct TransparentEmptyStructure h,
          TransparentPointerWrappingStructure i,
          struct StructWithAssociatedConstantInImpl j,
          struct EnumWithAssociatedConstantInImpl k);

void erased_root(uint32_t *a,
                 uint32_t *b,
                 TransparentPrimitiveWrappingStructure c,
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

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
