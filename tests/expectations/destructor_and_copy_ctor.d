module cbindgen;

@nogc nothrow @safe:

enum FillRule : ubyte {
  A,
  B,
}

/// This will have a destructor manually implemented via variant_body, and
/// similarly a Drop impl in Rust.
struct OwnedSlice(T) {
  @disable this();
  ulong len;
  T *ptr;
}

struct Polygon(LengthPercentage) {
  @disable this();
  FillRule fill;
  OwnedSlice!(LengthPercentage) coordinates;
}

enum Foo_Tag : ubyte {
  Bar,
  Polygon1,
  Slice1,
  Slice2,
  Slice3,
  Slice4,
}

struct Slice3_Body(T) {
  @disable this();
  FillRule fill;
  OwnedSlice!(T) coords;
}

struct Slice4_Body(T) {
  @disable this();
  FillRule fill;
  OwnedSlice!(int) coords;
}

struct Foo {
  Foo_Tag tag;
  union {
    struct {
      Polygon!(T) polygon1;
    };
    struct {
      OwnedSlice!(T) slice1;
    };
    struct {
      OwnedSlice!(int) slice2;
    };
    Slice3_Body slice3;
    Slice4_Body slice4;
  };
}

enum Baz_Tag : ubyte {
  Bar2,
  Polygon21,
  Slice21,
  Slice22,
  Slice23,
  Slice24,
}

struct Slice23_Body(T) {
  @disable this();
  Baz_Tag tag;
  FillRule fill;
  OwnedSlice!(T) coords;
}

struct Slice24_Body(T) {
  @disable this();
  Baz_Tag tag;
  FillRule fill;
  OwnedSlice!(int) coords;
}

union Baz {
  Baz_Tag tag;
  struct {
    Baz_Tag polygon21_tag;
    Polygon!(T) polygon21;
  };
  struct {
    Baz_Tag slice21_tag;
    OwnedSlice!(T) slice21;
  };
  struct {
    Baz_Tag slice22_tag;
    OwnedSlice!(int) slice22;
  };
  Slice23_Body slice23;
  Slice24_Body slice24;
}

enum Taz_Tag : ubyte {
  Bar3,
  Taz1,
  Taz3,
}

union Taz {
  Taz_Tag tag;
  struct {
    Taz_Tag taz1_tag;
    int taz1;
  };
  struct {
    Taz_Tag taz3_tag;
    OwnedSlice!(int) taz3;
  };
}

enum Tazz_Tag : ubyte {
  Bar4,
  Taz2,
}

union Tazz {
  Tazz_Tag tag;
  struct {
    Tazz_Tag taz2_tag;
    int taz2;
  };
}

enum Tazzz_Tag : ubyte {
  Bar5,
  Taz5,
}

union Tazzz {
  Tazzz_Tag tag;
  struct {
    Tazzz_Tag taz5_tag;
    int taz5;
  };
}

enum Tazzzz_Tag : ubyte {
  Taz6,
  Taz7,
}

union Tazzzz {
  Tazzzz_Tag tag;
  struct {
    Tazzzz_Tag taz6_tag;
    int taz6;
  };
  struct {
    Tazzzz_Tag taz7_tag;
    uint taz7;
  };
}

enum Qux_Tag : ubyte {
  Qux1,
  Qux2,
}

union Qux {
  Qux_Tag tag;
  struct {
    Qux_Tag qux1_tag;
    int qux1;
  };
  struct {
    Qux_Tag qux2_tag;
    uint qux2;
  };
}

extern(C) {

void root(const Foo!(uint) *a,
          const Baz!(int) *b,
          const Taz *c,
          Tazz d,
          const Tazzz *e,
          const Tazzzz *f,
          const Qux *g);

}  // extern(C)
