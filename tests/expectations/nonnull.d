module cbindgen;

@nogc nothrow @safe:

struct Opaque;

struct Foo(T) {
  @disable this();
  float *a;
  T *b;
  Opaque *c;
  T **d;
  float **e;
  Opaque **f;
  T *g;
  int *h;
  int **i;
}

extern(C) {

void root(int *arg, Foo!(ulong) *foo, Opaque **d);

}  // extern(C)
