module cbindgen;

@nogc nothrow @safe:

enum DeprecatedEnum : int {
  A = 0,
}

enum DeprecatedEnumWithNote : int {
  B = 0,
}

enum EnumWithDeprecatedVariants : int {
  C = 0,
  D = 1,
  E = 2,
  F = 3,
}

struct DeprecatedStruct {
  @disable this();
  int a;
}

struct DeprecatedStructWithNote {
  @disable this();
  int a;
}

enum EnumWithDeprecatedStructVariants_Tag : ubyte {
  Foo,
  Bar,
  Baz,
}

struct Bar_Body {
  @disable this();
  EnumWithDeprecatedStructVariants_Tag tag;
  ubyte x;
  short y;
}

struct Baz_Body {
  @disable this();
  EnumWithDeprecatedStructVariants_Tag tag;
  ubyte x;
  ubyte y;
}

union EnumWithDeprecatedStructVariants {
  EnumWithDeprecatedStructVariants_Tag tag;
  struct {
    EnumWithDeprecatedStructVariants_Tag foo_tag;
    short foo;
  };
  Bar_Body bar;
  Baz_Body baz;
}

extern(C) {

void deprecated_without_note();

void deprecated_without_bracket();

void deprecated_with_note();

void deprecated_with_note_and_since();

void deprecated_with_note_which_requires_to_be_escaped();

void dummy(DeprecatedEnum a,
           DeprecatedEnumWithNote b,
           EnumWithDeprecatedVariants c,
           DeprecatedStruct d,
           DeprecatedStructWithNote e,
           EnumWithDeprecatedStructVariants f);

}  // extern(C)
