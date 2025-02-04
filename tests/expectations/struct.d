module cbindgen;

@nogc nothrow @safe:

struct Opaque;

struct Normal {
  @disable this();
  int x;
  float y;
}

struct NormalWithZST {
  @disable this();
  int x;
  float y;
}

struct TupleRenamed {
  @disable this();
  int m0;
  float m1;
}

struct TupleNamed {
  @disable this();
  int x;
  float y;
}

extern(C) {

void root(Opaque *a, Normal b, NormalWithZST c, TupleRenamed d, TupleNamed e);

}  // extern(C)
