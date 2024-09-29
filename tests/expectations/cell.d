module cbindgen;

@nogc nothrow @safe:

struct NotReprC(T = void);

struct RefCell(T = void);

alias Foo = NotReprC!(RefCell!(int));

struct MyStruct {
  @disable this();
  int number;
}

extern(C) {

void root(const Foo *a, const MyStruct *with_cell);

}  // extern(C)
