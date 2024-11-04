module cbindgen;

@nogc nothrow @safe:

struct Foo(T, U) {
  @disable this();
  T x;
  U y;
}

alias IntFoo(T) = Foo!(int, T);

extern(C) {

void root(IntFoo!(int) a);

}  // extern(C)
