module cbindgen;

@nogc nothrow @safe:

struct Foo(T) {
  @disable this();
  const int *something;
}

struct Bar {
  @disable this();
  int something;
  Foo!(Bar) subexpressions;
}

extern(C) {

void root(Bar b);

}  // extern(C)
