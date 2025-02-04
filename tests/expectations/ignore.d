module cbindgen;

@nogc nothrow @safe:

enum NO_IGNORE_CONST = 0;

enum NoIgnoreStructWithImpl_NO_IGNORE_INNER_CONST = 0;

extern(C) {

void no_ignore_root();

void no_ignore_associated_method();

}  // extern(C)
