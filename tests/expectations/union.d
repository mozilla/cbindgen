module cbindgen;

@nogc nothrow @safe:

struct Opaque;

union Normal {
  int x;
  float y;
}

union NormalWithZST {
  int x;
  float y;
}

extern(C) {

void root(Opaque *a, Normal b, NormalWithZST c);

}  // extern(C)
