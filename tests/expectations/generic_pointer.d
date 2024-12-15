module cbindgen;

@nogc nothrow @safe:

struct Foo(T) {
  @disable this();
  T a;
}

alias Boo = Foo!(ubyte*);

extern(C) {

void root(Boo x);

void my_function(Foo!(ubyte[4] ) x);

}  // extern(C)
