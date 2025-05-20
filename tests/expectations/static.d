module cbindgen;

@nogc nothrow @safe:

struct Bar;

struct Foo {
  @disable this();

}

extern(C) {

extern const int NUMBER;

extern Foo FOO;

extern const Bar BAR;

void root();

}  // extern(C)
