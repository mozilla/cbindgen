module cbindgen;

@nogc nothrow @safe:

enum FOUR = 4;

enum E : byte {
  A = 1,
  B = -1,
  C = (1 + 2),
  D = FOUR,
  F = 5,
  G = cast(byte)54,
  H = cast(byte)false,
}

extern(C) {

void root(const E*);

}  // extern(C)
