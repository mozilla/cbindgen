module cbindgen;

@nogc nothrow @safe:

struct Point(T) {
  @disable this();
  T x;
  T y;
}

enum Foo_Tag : ubyte {
  Foo,
  Bar,
  Baz,
  Bazz,
}

struct Foo_Body(T) {
  @disable this();
  Foo_Tag tag;
  int x;
  Point!(T) y;
  Point!(float) z;
}

union Foo {
  Foo_Tag tag;
  Foo_Body foo;
  struct {
    Foo_Tag bar_tag;
    T bar;
  };
  struct {
    Foo_Tag baz_tag;
    Point!(T) baz;
  };
}

enum Bar_Tag {
  Bar1,
  Bar2,
  Bar3,
  Bar4,
}

struct Bar1_Body(T) {
  @disable this();
  int x;
  Point!(T) y;
  Point!(float) z;
  int  function(int) u;
}

struct Bar {
  Bar_Tag tag;
  union {
    Bar1_Body bar1;
    struct {
      T bar2;
    };
    struct {
      Point!(T) bar3;
    };
  };
}

enum Baz_Tag : ubyte {
  Baz1,
  Baz2,
  Baz3,
}

union Baz {
  Baz_Tag tag;
  struct {
    Baz_Tag baz1_tag;
    Bar!(uint) baz1;
  };
  struct {
    Baz_Tag baz2_tag;
    Point!(int) baz2;
  };
}

enum Taz_Tag : ubyte {
  Taz1,
  Taz2,
  Taz3,
}

struct Taz {
  Taz_Tag tag;
  union {
    struct {
      Bar!(uint) taz1;
    };
    struct {
      Baz taz2;
    };
  };
}

extern(C) {

void foo(const Foo!(int) *foo, const Bar!(int) *bar, const Baz *baz, const Taz *taz);

}  // extern(C)
