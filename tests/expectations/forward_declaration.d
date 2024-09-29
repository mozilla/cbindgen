module cbindgen;

@nogc nothrow @safe:

struct StructInfo {
  @disable this();
  const TypeInfo *const *fields;
  ulong num_fields;
}

enum TypeData_Tag {
  Primitive,
  Struct,
}

struct TypeData {
  TypeData_Tag tag;
  union {
    struct {
      StructInfo struct_;
    };
  };
}

struct TypeInfo {
  @disable this();
  TypeData data;
}

extern(C) {

void root(TypeInfo x);

}  // extern(C)
