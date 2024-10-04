module cbindgen;

@nogc nothrow @safe:

struct NotReprC(T = void);

alias Foo = NotReprC!(int*);

struct MyStruct {
  @disable this();
  int *number;
}

extern(C) {

void root(const Foo *a, const MyStruct *with_box);

void drop_box(int *x);

void drop_box_opt(int *x);

}  // extern(C)
