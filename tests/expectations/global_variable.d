module cbindgen;

@nogc nothrow @safe:

extern(C) {

extern char [128] MUT_GLOBAL_ARRAY;

extern const char [128] CONST_GLOBAL_ARRAY;

}  // extern(C)
