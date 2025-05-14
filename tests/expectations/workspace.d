module cbindgen;

@nogc nothrow @safe:

enum EXT_CONST = 0;

struct ExtType {
  @disable this();
  uint data;
}

extern(C) {

void consume_ext(ExtType _ext);

}  // extern(C)
