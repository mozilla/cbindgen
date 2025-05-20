module cbindgen;

@nogc nothrow @safe:

struct A;

struct B {
  @disable this();
  int x;
  float y;
}

extern(C) {

void root(const A *a, B b);

}  // extern(C)
