module cbindgen;

@nogc nothrow @safe:

enum H = 10;

enum E : ubyte {
  x = 0,
  y = 1,
}

struct A;

struct C;

struct B {
  @disable this();
  int x;
  float y;
}

union D {
  int x;
  float y;
}

alias F = A;

enum I = cast(long)cast(F*)10;

extern(C) {

extern const int G;

void root(const A *a, B b, C c, D d, E e, F f);

}  // extern(C)
