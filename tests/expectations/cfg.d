module cbindgen;

@nogc nothrow @safe:

enum BarType : uint {
  A,
  B,
  C,
}

enum FooType : uint {
  A,
  B,
  C,
}

struct FooHandle {
  @disable this();
  FooType ty;
  int x;
  float y;
}

enum C_Tag : ubyte {
  C1,
  C2,
  C3,
  C5,
}

struct C5_Body {
  @disable this();
  C_Tag tag;
  int int_;
}

union C {
  C_Tag tag;
  C5_Body c5;
}

struct BarHandle {
  @disable this();
  BarType ty;
  int x;
  float y;
}

struct ConditionalField {
  @disable this();
  int field;
}

struct Normal {
  @disable this();
  int x;
  float y;
}

extern(C) {

extern int [2] global_array_with_different_sizes;

extern int [1] global_array_with_different_sizes;

void root(FooHandle a, C c);

void root(BarHandle a, C c);

void cond(ConditionalField a);

extern int foo();

extern void bar(Normal a);

}  // extern(C)
