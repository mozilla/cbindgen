module cbindgen;

@nogc nothrow @safe:

struct Option(T = void);

alias Foo = ulong;

struct Bar {
  @disable this();
  Option!(Foo) foo;
}

extern(C) {

void root(Bar f);

}  // extern(C)
