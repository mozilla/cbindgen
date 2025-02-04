module cbindgen;

@nogc nothrow @safe:

enum FOO = 10;

enum ZOM = 3.14;

struct Foo {
  @disable this();
  int [FOO] x;
}

extern(C) {

void root(Foo x);

}  // extern(C)
