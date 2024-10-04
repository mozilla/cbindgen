module cbindgen;

@nogc nothrow @safe:

struct A {
  @disable this();
  const int *data;
}

enum E_Tag {
  V,
  U,
}

struct E {
  E_Tag tag;
  union {
    struct {
      const ubyte *u;
    };
  };
}

extern(C) {

void root(A _a, E _e);

}  // extern(C)
