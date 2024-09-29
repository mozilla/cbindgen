module cbindgen;

@nogc nothrow @safe:

enum Bar {
  Bar_Some,
  Bar_Thing,
}

struct Foo(T) {
  @disable this();
  T a;
}

alias Boo = Foo!(ubyte);

extern(C) {

void root(Boo x, Bar y);

void unsafe_root(Boo x, Bar y);

}  // extern(C)
