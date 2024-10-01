module cbindgen;

@nogc nothrow @safe:

struct Foo {
  @disable this();
  uint a;
}

extern(C) {

void root(Foo a);

}  // extern(C)
