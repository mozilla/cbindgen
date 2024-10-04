module cbindgen;

@nogc nothrow @safe:

struct PinTest {
  @disable this();
  int *pinned_box;
  int *pinned_ref;
}

extern(C) {

void root(int *s, PinTest p);

}  // extern(C)
