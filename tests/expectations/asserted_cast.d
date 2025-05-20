module cbindgen;

@nogc nothrow @safe:

struct I;

enum H_Tag : ubyte {
  H_Foo,
  H_Bar,
  H_Baz,
}

struct H_Bar_Body {
  @disable this();
  ubyte x;
  short y;
}

struct H {
  H_Tag tag;
  union {
    struct {
      short foo;
    };
    H_Bar_Body bar;
  };
}

enum J_Tag : ubyte {
  J_Foo,
  J_Bar,
  J_Baz,
}

struct J_Bar_Body {
  @disable this();
  ubyte x;
  short y;
}

struct J {
  J_Tag tag;
  union {
    struct {
      short foo;
    };
    J_Bar_Body bar;
  };
}

enum K_Tag : ubyte {
  K_Foo,
  K_Bar,
  K_Baz,
}

struct K_Bar_Body {
  @disable this();
  K_Tag tag;
  ubyte x;
  short y;
}

union K {
  K_Tag tag;
  struct {
    K_Tag foo_tag;
    short foo;
  };
  K_Bar_Body bar;
}

extern(C) {

void foo(H h, I i, J j, K k);

}  // extern(C)
