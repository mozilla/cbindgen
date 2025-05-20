module cbindgen;

@nogc nothrow @safe:

struct ArrayVec(T, ulong CAP) {
  @disable this();
  T [CAP] xs;
  uint len;
}

extern(C) {

int push(ArrayVec!(ubyte*, 100) *v, ubyte *elem);

}  // extern(C)
