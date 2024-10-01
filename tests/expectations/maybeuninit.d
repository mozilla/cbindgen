module cbindgen;

@nogc nothrow @safe:

(T = void)struct NotReprC;

alias Foo = NotReprC!(const int*);

struct MyStruct {
  @disable this();
  const int *number;
}

extern(C) {

void root(const Foo *a, const MyStruct *with_maybe_uninit);

}  // extern(C)
