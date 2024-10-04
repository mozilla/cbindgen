module cbindgen;

@nogc nothrow @safe:

enum E {
  V,
}

struct S {
  @disable this();
  ubyte field;
}

alias A = ubyte;

enum C1 = S(field: 0);

enum C2 = V;

enum C3 = 0;
