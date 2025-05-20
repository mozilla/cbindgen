module cbindgen;

@nogc nothrow @safe:

struct A {
  @disable this();
  int namespace_;
  float float_;
}

struct B {
  @disable this();
  int namespace_;
  float float_;
}

enum C_Tag : ubyte {
  D,
}

struct D_Body {
  @disable this();
  int namespace_;
  float float_;
}

struct C {
  C_Tag tag;
  union {
    D_Body d;
  };
}

enum E_Tag : ubyte {
  Double,
  Float,
}

struct E {
  E_Tag tag;
  union {
    struct {
      double double_;
    };
    struct {
      float float_;
    };
  };
}

enum F_Tag : ubyte {
  double_,
  float_,
}

struct F {
  F_Tag tag;
  union {
    struct {
      double double_;
    };
    struct {
      float float_;
    };
  };
}

extern(C) {

void root(A a, B b, C c, E e, F f, int namespace_, float float_);

}  // extern(C)
