module cbindgen;

@nogc nothrow @safe:

enum EXPORT_ME_TOO = 42;

struct ExportMe {
  @disable this();
  ulong val;
}

struct ExportMe2 {
  @disable this();
  ulong val;
}

extern(C) {

void export_me(ExportMe *val);

void export_me_2(ExportMe2*);

void from_really_nested_mod();

}  // extern(C)
