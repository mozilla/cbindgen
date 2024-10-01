module cbindgen;

@nogc nothrow @safe:

enum A : ubyte {
  A1,
  A2,
  A3,
}

enum B : ubyte {
  B1,
  B2,
  B3,
}

enum C_Tag : ubyte {
  C1,
  C2,
  C3,
}

struct C1_Body {
  @disable this();
  C_Tag tag;
  uint a;
}

struct C2_Body {
  @disable this();
  C_Tag tag;
  uint b;
}

union C {
  C_Tag tag;
  C1_Body c1;
  C2_Body c2;
}

extern(C) {

void root(A a, B b, C c);

}  // extern(C)
