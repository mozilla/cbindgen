module cbindgen;

@nogc nothrow @safe:

struct HasBitfields {
  @disable this();
  ulong foo: 8;
  ulong bar: 56;
}

extern(C) {

void root(const HasBitfields*);

}  // extern(C)
