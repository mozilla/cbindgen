module cbindgen;

@nogc nothrow @safe:

struct Bar {
  @disable this();
  int a;
}

struct Foo {
  @disable this();
  int a;
  uint b;
  Bar bar;
}

enum VAL = Foo(a: 42, b: 1337, bar: Bar(a: 323));

extern(C) {

void root(Foo x);

}  // extern(C)
