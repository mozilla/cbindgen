module cbindgen;

@nogc nothrow @safe:

struct Inner(ulong N) {
  @disable this();
  ubyte [N] bytes;
}

struct Outer(ulong N) {
  @disable this();
  Inner!(N) inner;
}

extern(C) {

Outer!(1) one();

Outer!(2) two();

}  // extern(C)
