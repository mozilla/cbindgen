#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct DummyStruct;

struct EnumWithAssociatedConstantInImpl;

typedef struct DummyStruct TransparentComplexWrappingStructTuple;

typedef uint32_t TransparentPrimitiveWrappingStructTuple;

typedef struct DummyStruct TransparentComplexWrappingStructure;

typedef uint32_t TransparentPrimitiveWrappingStructure;

typedef struct DummyStruct TransparentComplexWrapper_i32;

typedef uint32_t TransparentPrimitiveWrapper_i32;

typedef uint32_t TransparentPrimitiveWithAssociatedConstants;
#define TransparentPrimitiveWithAssociatedConstants_ZERO 0
#define TransparentPrimitiveWithAssociatedConstants_ONE 1

#define EnumWithAssociatedConstantInImpl_TEN 10

void root(TransparentComplexWrappingStructTuple a,
          TransparentPrimitiveWrappingStructTuple b,
          TransparentComplexWrappingStructure c,
          TransparentPrimitiveWrappingStructure d,
          TransparentComplexWrapper_i32 e,
          TransparentPrimitiveWrapper_i32 f,
          TransparentPrimitiveWithAssociatedConstants g,
          struct EnumWithAssociatedConstantInImpl h);
