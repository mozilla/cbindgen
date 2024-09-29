module cbindgen;

@nogc nothrow @safe:

enum FONT_WEIGHT_FRACTION_BITS = 6;

struct FixedPoint(ushort FRACTION_BITS) {
  @disable this();
  ushort value;
}

alias FontWeightFixedPoint = FixedPoint!(FONT_WEIGHT_FRACTION_BITS);

struct FontWeight {
  @disable this();
  FontWeightFixedPoint _0;
}
enum FontWeight_NORMAL = FontWeight(_0: FontWeightFixedPoint(value: (400 << FONT_WEIGHT_FRACTION_BITS)));

extern(C) {

void root(FontWeight w);

}  // extern(C)
