module cbindgen;

@nogc nothrow @safe:

struct Opaque;

struct References {
  @disable this();
  const Opaque *a;
  Opaque *b;
  const Opaque *c;
  Opaque *d;
}

struct Pointers(T) {
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
  const T *j;
  T *k;
}

extern(C) {

void value_arg(References arg);

void mutltiple_args(int *arg, Pointers!(ulong) *foo, Opaque **d);

void ref_arg(const Pointers!(ulong) *arg);

void mut_ref_arg(Pointers!(ulong) *arg);

void optional_ref_arg(const Pointers!(ulong) *arg);

void optional_mut_ref_arg(Pointers!(ulong) *arg);

void nullable_const_ptr(const Pointers!(ulong) *arg);

void nullable_mut_ptr(Pointers!(ulong) *arg);

}  // extern(C)
