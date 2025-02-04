module cbindgen;

@nogc nothrow @safe:

enum C : uint {
  X = 2,
  Y,
}

struct A {
  @disable this();
  int _0;
}

struct B {
  @disable this();
  int x;
  float y;
}

struct D {
  @disable this();
  ubyte List;
  ulong Of;
  B Things;
}

enum F_Tag : ubyte {
  Foo,
  Bar,
  Baz,
}

struct Bar_Body {
  @disable this();
  F_Tag tag;
  ubyte x;
  short y;
}

union F {
  F_Tag tag;
  struct {
    F_Tag foo_tag;
    short foo;
  };
  Bar_Body bar;
}

enum H_Tag : ubyte {
  Hello,
  There,
  Everyone,
}

struct There_Body {
  @disable this();
  ubyte x;
  short y;
}

struct H {
  H_Tag tag;
  union {
    struct {
      short hello;
    };
    There_Body there;
  };
}

enum I_Tag : ubyte {
  ThereAgain,
  SomethingElse,
}

struct ThereAgain_Body {
  @disable this();
  ubyte x;
  short y;
}

struct I {
  I_Tag tag;
  union {
    ThereAgain_Body there_again;
  };
}

extern(C) {

void root(A a, B b, C c, D d, F f, H h, I i);

}  // extern(C)
