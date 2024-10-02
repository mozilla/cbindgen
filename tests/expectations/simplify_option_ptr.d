module cbindgen;

@nogc nothrow @safe:

struct Opaque;

(T = void)struct Option;

struct Foo {
  @disable this();
  const Opaque *x;
  Opaque *y;
  void  function() z;
  void  function() zz;
}

union Bar {
  const Opaque *x;
  Opaque *y;
  void  function() z;
  void  function() zz;
}

extern(C) {

void root(const Opaque *a,
          Opaque *b,
          Foo c,
          Bar d,
          Option!(Opaque*) *e,
          void  function(const Opaque*) f);

}  // extern(C)
