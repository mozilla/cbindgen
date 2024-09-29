module cbindgen;

@nogc nothrow @safe:

enum MY_CONST = 4;

extern(C) {

void ExternFunction();

}  // extern(C)
