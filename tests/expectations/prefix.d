module cbindgen;

@nogc nothrow @safe:

enum LEN = 22;

enum X = (22 << 22);

enum Y = (X + X);

alias NamedLenArray = int[LEN] ;

alias ValuedLenArray = int[22] ;

enum AbsoluteFontWeight_Tag : ubyte {
  Weight,
  Normal,
  Bold,
}

union AbsoluteFontWeight {
  AbsoluteFontWeight_Tag tag;
  struct {
    AbsoluteFontWeight_Tag weight_tag;
    float weight;
  };
}

extern(C) {

void root(NamedLenArray x, ValuedLenArray y, AbsoluteFontWeight z);

}  // extern(C)
