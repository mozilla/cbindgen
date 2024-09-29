module cbindgen;

@nogc nothrow @safe:

enum Foo_Tag {
  A,
}

struct Foo {
  Foo_Tag tag;
  union {
    struct {
      float [20] a;
    };
  };
}

extern(C) {

void root(Foo a);

}  // extern(C)
