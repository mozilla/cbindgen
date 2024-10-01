module cbindgen;

@nogc nothrow @safe:

extern(C) {

void unnamed(const ulong*);

void pointer_test(const ulong *a);

void print_from_rust();

}  // extern(C)
