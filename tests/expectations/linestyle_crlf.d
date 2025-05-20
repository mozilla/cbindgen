module cbindgen;

@nogc nothrow @safe:

struct Dummy {
  @disable this();
  int x;
  float y;
}

extern(C) {

void root(Dummy d);

}  // extern(C)
