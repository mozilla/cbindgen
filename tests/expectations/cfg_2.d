module cbindgen;

@nogc nothrow @safe:

enum DEFAULT_X = 8;

enum DEFAULT_X = 42;

struct Foo {
  @disable this();
  int x;
}

struct Bar {
  @disable this();
  Foo y;
}

struct Bar {
  @disable this();
  Foo z;
}

struct Root {
  @disable this();
  Bar w;
}

extern(C) {

void root(Root a);

}  // extern(C)
