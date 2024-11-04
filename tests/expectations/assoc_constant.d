module cbindgen;

@nogc nothrow @safe:

struct Foo {
  @disable this();

}
enum Foo_GA = 10;
enum Foo_ZO = 3.14;

extern(C) {

void root(Foo x);

}  // extern(C)
