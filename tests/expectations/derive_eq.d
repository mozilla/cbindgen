module cbindgen;

@nogc nothrow @safe:

struct Foo {
  @disable this();
  bool a;
  int b;
}

enum Bar_Tag : ubyte {
  Baz,
  Bazz,
  FooNamed,
  FooParen,
}

struct Bazz_Body {
  @disable this();
  Bar_Tag tag;
  Foo named;
}

struct FooNamed_Body {
  @disable this();
  Bar_Tag tag;
  int different;
  uint fields;
}

struct FooParen_Body {
  @disable this();
  Bar_Tag tag;
  int _0;
  Foo _1;
}

union Bar {
  Bar_Tag tag;
  Bazz_Body bazz;
  FooNamed_Body foo_named;
  FooParen_Body foo_paren;
}

extern(C) {

Foo root(Bar aBar);

}  // extern(C)
