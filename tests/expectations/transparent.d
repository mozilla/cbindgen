module cbindgen;

@nogc nothrow @safe:

struct DummyStruct;

struct EnumWithAssociatedConstantInImpl;

alias TransparentComplexWrappingStructTuple = DummyStruct;

alias TransparentPrimitiveWrappingStructTuple = uint;

alias TransparentComplexWrappingStructure = DummyStruct;

alias TransparentPrimitiveWrappingStructure = uint;

alias TransparentComplexWrapper(T) = DummyStruct;

alias TransparentPrimitiveWrapper(T) = uint;

alias TransparentPrimitiveWithAssociatedConstants = uint;
enum TransparentPrimitiveWithAssociatedConstants_ZERO = 0;
enum TransparentPrimitiveWithAssociatedConstants_ONE = 1;

struct TransparentEmptyStructure {
  @disable this();

}

enum EnumWithAssociatedConstantInImpl_TEN = 10;

extern(C) {

void root(TransparentComplexWrappingStructTuple a,
          TransparentPrimitiveWrappingStructTuple b,
          TransparentComplexWrappingStructure c,
          TransparentPrimitiveWrappingStructure d,
          TransparentComplexWrapper!(int) e,
          TransparentPrimitiveWrapper!(int) f,
          TransparentPrimitiveWithAssociatedConstants g,
          TransparentEmptyStructure h,
          EnumWithAssociatedConstantInImpl i);

}  // extern(C)
