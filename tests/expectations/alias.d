module cbindgen;

@nogc nothrow @safe:

enum Status : uint {
  Ok,
  Err,
}

struct Dep {
  @disable this();
  int a;
  float b;
}

struct Foo(X) {
  @disable this();
  X a;
  X b;
  Dep c;
}

alias IntFoo = Foo!(int);

alias DoubleFoo = Foo!(double);

alias Unit = int;

alias SpecialStatus = Status;

extern(C) {

void root(IntFoo x, DoubleFoo y, Unit z, SpecialStatus w);

}  // extern(C)
