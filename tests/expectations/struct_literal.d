module cbindgen;

@nogc nothrow @safe:

struct Bar;

struct Foo {
  @disable this();
  int a;
  uint b;
}
enum Foo_FOO = Foo(a: 42, b: 47);
enum Foo_FOO2 = Foo(a: 42, b: 47);
enum Foo_FOO3 = Foo(a: 42, b: 47);


enum BAR = Foo(a: 42, b: 1337);



extern(C) {

void root(Foo x, Bar bar);

}  // extern(C)
