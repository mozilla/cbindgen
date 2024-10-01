module cbindgen;

@nogc nothrow @safe:

enum Enum : ubyte {
  a,
  b,
}

struct Struct {
  @disable this();
  Enum field;
}

extern(C) {

extern const Enum STATIC;

void fn(Struct arg);

}  // extern(C)
