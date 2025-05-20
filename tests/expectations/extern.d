module cbindgen;

@nogc nothrow @safe:

struct Normal {
  @disable this();
  int x;
  float y;
}

extern(C) {

extern int foo();

extern void bar(Normal a);

extern int baz();

}  // extern(C)
