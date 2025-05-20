module cbindgen;

@nogc nothrow @safe:

struct Foo {
  @disable this();

}

extern(C) {

void root(Foo a);

}  // extern(C)
