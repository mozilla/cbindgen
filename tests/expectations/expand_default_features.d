module cbindgen;

@nogc nothrow @safe:

struct Foo {
  @disable this();

}

extern(C) {

void extra_debug_fn();

void root(Foo a);

}  // extern(C)
