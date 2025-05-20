module cbindgen;

@nogc nothrow @safe:

struct TraitObject {
  @disable this();
  void *data;
  void *vtable;
}

extern(C) {

void *root(const void *ptr, TraitObject t);

}  // extern(C)
