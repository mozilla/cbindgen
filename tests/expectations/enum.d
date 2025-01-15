module cbindgen;

@nogc nothrow @safe:

enum A : ulong {
  a1 = 0,
  a2 = 2,
  a3,
  a4 = 5,
}

enum B : uint {
  b1 = 0,
  b2 = 2,
  b3,
  b4 = 5,
}

enum C : ushort {
  c1 = 0,
  c2 = 2,
  c3,
  c4 = 5,
}

enum D : ubyte {
  d1 = 0,
  d2 = 2,
  d3,
  d4 = 5,
}

enum E : ulong {
  e1 = 0,
  e2 = 2,
  e3,
  e4 = 5,
}

enum F : long {
  f1 = 0,
  f2 = 2,
  f3,
  f4 = 5,
}

enum L {
  l1,
  l2,
  l3,
  l4,
}

enum M : byte {
  m1 = -1,
  m2 = 0,
  m3 = 1,
}

enum N {
  n1,
  n2,
  n3,
  n4,
}

enum O : byte {
  o1,
  o2,
  o3,
  o4,
}

struct J;

struct K;

struct Opaque;

enum G_Tag : ubyte {
  Foo,
  Bar,
  Baz,
}

struct Bar_Body {
  @disable this();
  G_Tag tag;
  ubyte x;
  short y;
}

union G {
  G_Tag tag;
  struct {
    G_Tag foo_tag;
    short foo;
  };
  Bar_Body bar;
}

enum H_Tag {
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

enum I_Tag : ubyte {
  I_Foo,
  I_Bar,
  I_Baz,
}

struct I_Bar_Body {
  @disable this();
  ubyte x;
  short y;
}

struct I {
  I_Tag tag;
  union {
    struct {
      short foo;
    };
    I_Bar_Body bar;
  };
}

enum P_Tag : ubyte {
  P0,
  P1,
}

struct P1_Body {
  @disable this();
  ubyte _0;
  ubyte _1;
  ubyte _2;
}

struct P {
  P_Tag tag;
  union {
    struct {
      ubyte p0;
    };
    P1_Body p1;
  };
}

enum Q_Tag {
  Ok,
  Err,
}

struct Q {
  Q_Tag tag;
  union {
    struct {
      uint *ok;
    };
    struct {
      uint err;
    };
  };
}

enum R_Tag {
  IRFoo,
  IRBar,
  IRBaz,
}

struct IRBar_Body {
  @disable this();
  ubyte x;
  short y;
}

struct R {
  R_Tag tag;
  union {
    struct {
      short IRFoo;
    };
    IRBar_Body IRBar;
  };
}

extern(C) {

void root(Opaque *opaque,
          A a,
          B b,
          C c,
          D d,
          E e,
          F f,
          G g,
          H h,
          I i,
          J j,
          K k,
          L l,
          M m,
          N n,
          O o,
          P p,
          Q q,
          R r);

}  // extern(C)
