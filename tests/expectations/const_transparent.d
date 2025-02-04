module cbindgen;

@nogc nothrow @safe:

alias Wrapper(T) = T;

alias TransparentStruct = ubyte;
enum TransparentStruct_ASSOC_STRUCT_FOO = 1;
enum TransparentStruct_ASSOC_STRUCT_BAR = 2;
enum TransparentStruct_ASSOC_STRUCT_BAZ = 3;

alias TransparentTupleStruct = ubyte;

alias TransparentStructWithErasedField(T) = Wrapper!(T);

enum STRUCT_FOO = 4;

enum STRUCT_BAR = 5;

enum STRUCT_BAZ = 6;

enum COMPLEX = 7;
