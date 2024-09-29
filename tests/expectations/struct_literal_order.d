module cbindgen;

@nogc nothrow @safe:

struct ABC {
  @disable this();
  float a;
  uint b;
  uint c;
}
enum ABC_abc = ABC(a: 1.0, b: 2, c: 3);
enum ABC_bac = ABC(a: 1.0, b: 2, c: 3);
enum ABC_cba = ABC(a: 1.0, b: 2, c: 3);

struct BAC {
  @disable this();
  uint b;
  float a;
  int c;
}
enum BAC_abc = BAC(b: 1, a: 2.0, c: 3);
enum BAC_bac = BAC(b: 1, a: 2.0, c: 3);
enum BAC_cba = BAC(b: 1, a: 2.0, c: 3);

extern(C) {

void root(ABC a1, BAC a2);

}  // extern(C)
