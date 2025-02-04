module cbindgen;

@nogc nothrow @safe:

enum EXPORT_ME_TOO = 42;

struct ExportMe {
  @disable this();
  ulong val;
}

extern(C) {

void export_me(ExportMe *val);

}  // extern(C)
