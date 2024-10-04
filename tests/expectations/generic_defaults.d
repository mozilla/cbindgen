module cbindgen;

@nogc nothrow @safe:

alias Foo(T, P = void) = T;

struct Bar(T, P) {
  @disable this();
  Foo!(T) f;
  P p;
}

alias Baz(T) = Foo!(T);

extern(C) {

void foo_root(Foo!(short) f, Bar!(int, uint) b, Baz!(long) z);

}  // extern(C)
