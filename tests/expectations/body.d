module cbindgen;

@nogc nothrow @safe:

enum MyCLikeEnum {
  Foo1,
  Bar1,
  Baz1,
}

enum MyCLikeEnum_Prepended {
  Foo1_Prepended,
  Bar1_Prepended,
  Baz1_Prepended,
}

struct MyFancyStruct {
  @disable this();
  int i;
}

enum MyFancyEnum_Tag {
  Foo,
  Bar,
  Baz,
}

struct MyFancyEnum {
  MyFancyEnum_Tag tag;
  union {
    struct {
      int bar;
    };
    struct {
      int baz;
    };
  };
}

union MyUnion {
  float f;
  uint u;
}

struct MyFancyStruct_Prepended {
  @disable this();
  int i;
}

enum MyFancyEnum_Prepended_Tag {
  Foo_Prepended,
  Bar_Prepended,
  Baz_Prepended,
}

struct MyFancyEnum_Prepended {
  MyFancyEnum_Prepended_Tag tag;
  union {
    struct {
      int bar_prepended;
    };
    struct {
      int baz_prepended;
    };
  };
}

union MyUnion_Prepended {
  float f;
  uint u;
}

extern(C) {

void root(MyFancyStruct s,
          MyFancyEnum e,
          MyCLikeEnum c,
          MyUnion u,
          MyFancyStruct_Prepended sp,
          MyFancyEnum_Prepended ep,
          MyCLikeEnum_Prepended cp,
          MyUnion_Prepended up);

}  // extern(C)
