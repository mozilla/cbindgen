module cbindgen;

@nogc nothrow @safe:

struct Foo(T) {
  @disable this();
  const int *something;
}

enum Bar_Tag : ubyte {
  Min,
  Max,
  Other,
}

union Bar {
  Bar_Tag tag;
  struct {
    Bar_Tag min_tag;
    Foo!(Bar) min;
  };
  struct {
    Bar_Tag max_tag;
    Foo!(Bar) max;
  };
}

extern(C) {

void root(Bar b);

}  // extern(C)
