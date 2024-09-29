module cbindgen;

@nogc nothrow @safe:

struct dep_struct {
  @disable this();
  uint x;
  double y;
}

extern(C) {

uint get_x(const dep_struct *dep_struct);

}  // extern(C)
