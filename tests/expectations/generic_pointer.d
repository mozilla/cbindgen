module cbindgen;

@nogc nothrow @safe:

struct Foo(T) {
  @disable this();
  T a;
}

alias Boo = Foo!(ubyte*);

extern(C) {

void root(Boo x);

}  // extern(C)
