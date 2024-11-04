module cbindgen;

@nogc nothrow @safe:

enum BindingType : uint {
  Buffer = 0,
  NotBuffer = 1,
}

struct BindGroupLayoutEntry {
  @disable this();
  BindingType ty;
}

extern(C) {

void root(BindGroupLayoutEntry entry);

}  // extern(C)
