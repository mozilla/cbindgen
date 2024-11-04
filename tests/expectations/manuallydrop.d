module cbindgen;

@nogc nothrow @safe:

struct NotReprC(T = void);

struct Point {
  @disable this();
  int x;
  int y;
}

alias Foo = NotReprC!(Point);

struct MyStruct {
  @disable this();
  Point point;
}

extern(C) {

void root(const Foo *a, const MyStruct *with_manual_drop);

void take(Point with_manual_drop);

}  // extern(C)
