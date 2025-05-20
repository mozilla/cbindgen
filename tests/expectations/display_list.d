module cbindgen;

@nogc nothrow @safe:

struct Rect {
  @disable this();
  float x;
  float y;
  float w;
  float h;
}

struct Color {
  @disable this();
  ubyte r;
  ubyte g;
  ubyte b;
  ubyte a;
}

enum DisplayItem_Tag : ubyte {
  Fill,
  Image,
  ClearScreen,
}

struct Fill_Body {
  @disable this();
  DisplayItem_Tag tag;
  Rect _0;
  Color _1;
}

struct Image_Body {
  @disable this();
  DisplayItem_Tag tag;
  uint id;
  Rect bounds;
}

union DisplayItem {
  DisplayItem_Tag tag;
  Fill_Body fill;
  Image_Body image;
}

extern(C) {

bool push_item(DisplayItem item);

}  // extern(C)
