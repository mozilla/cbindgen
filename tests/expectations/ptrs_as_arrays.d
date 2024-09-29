module cbindgen;

@nogc nothrow @safe:

extern(C) {

void ptr_as_array(uint n, const uint [3] arg, const ulong *v);

void ptr_as_array1(uint n, const uint [3] arg, ulong [4] v);

void ptr_as_array2(uint n, uint [] arg, const ulong [] v);

void ptr_as_array_wrong_syntax(uint *arg, const uint *v, const uint*);

void ptr_as_array_unnamed(uint*, const uint*);

}  // extern(C)
