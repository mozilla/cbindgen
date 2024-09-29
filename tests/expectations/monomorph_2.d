module cbindgen;

@nogc nothrow @safe:

struct A;

struct B;

struct List(T) {
  @disable this();
  T *members;
  ulong count;
}

extern(C) {

void foo(List!(A) a);

void bar(List!(B) b);

}  // extern(C)
