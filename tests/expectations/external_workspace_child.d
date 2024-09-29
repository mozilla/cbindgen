module cbindgen;

@nogc nothrow @safe:

struct ExtType {
  @disable this();
  uint data;
}

extern(C) {

void consume_ext(ExtType _ext);

}  // extern(C)
