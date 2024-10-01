module cbindgen;

@nogc nothrow @safe:

struct Foo {
  @disable this();
  float x;
}

extern(C) {

void root(Foo a);

}  // extern(C)
