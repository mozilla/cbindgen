module cbindgen;

@nogc nothrow @safe:

struct Foo(T) {
  @disable this();
  const int *something;
}

union Bar {
  int something;
  Foo!(Bar) subexpressions;
}

extern(C) {

void root(Bar b);

}  // extern(C)
