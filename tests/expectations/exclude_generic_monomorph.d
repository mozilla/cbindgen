module cbindgen;

@nogc nothrow @safe:

(T = void)struct Option;

alias Foo = ulong;

struct Bar {
  @disable this();
  Option!(Foo) foo;
}

extern(C) {

void root(Bar f);

}  // extern(C)
