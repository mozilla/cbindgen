module cbindgen;

@nogc nothrow @safe:

struct LayoutUnit;

struct UnknownUnit;

struct TypedLength(T, Unit) {
  @disable this();
  T _0;
}

alias Length(T) = TypedLength!(T, UnknownUnit);

alias LayoutLength = TypedLength!(float, LayoutUnit);

struct TypedSideOffsets2D(T, U) {
  @disable this();
  T top;
  T right;
  T bottom;
  T left;
}

alias SideOffsets2D(T) = TypedSideOffsets2D!(T, UnknownUnit);

alias LayoutSideOffsets2D = TypedSideOffsets2D!(float, LayoutUnit);

struct TypedSize2D(T, U) {
  @disable this();
  T width;
  T height;
}

alias Size2D(T) = TypedSize2D!(T, UnknownUnit);

alias LayoutSize2D = TypedSize2D!(float, LayoutUnit);

struct TypedPoint2D(T, U) {
  @disable this();
  T x;
  T y;
}

alias Point2D(T) = TypedPoint2D!(T, UnknownUnit);

alias LayoutPoint2D = TypedPoint2D!(float, LayoutUnit);

struct TypedRect(T, U) {
  @disable this();
  TypedPoint2D!(T, U) origin;
  TypedSize2D!(T, U) size;
}

alias Rect(T) = TypedRect!(T, UnknownUnit);

alias LayoutRect = TypedRect!(float, LayoutUnit);

struct TypedTransform2D(T, Src, Dst) {
  @disable this();
  T m11;
  T m12;
  T m21;
  T m22;
  T m31;
  T m32;
}

extern(C) {

void root(TypedLength!(float, UnknownUnit) length_a,
          TypedLength!(float, LayoutUnit) length_b,
          Length!(float) length_c,
          LayoutLength length_d,
          TypedSideOffsets2D!(float, UnknownUnit) side_offsets_a,
          TypedSideOffsets2D!(float, LayoutUnit) side_offsets_b,
          SideOffsets2D!(float) side_offsets_c,
          LayoutSideOffsets2D side_offsets_d,
          TypedSize2D!(float, UnknownUnit) size_a,
          TypedSize2D!(float, LayoutUnit) size_b,
          Size2D!(float) size_c,
          LayoutSize2D size_d,
          TypedPoint2D!(float, UnknownUnit) point_a,
          TypedPoint2D!(float, LayoutUnit) point_b,
          Point2D!(float) point_c,
          LayoutPoint2D point_d,
          TypedRect!(float, UnknownUnit) rect_a,
          TypedRect!(float, LayoutUnit) rect_b,
          Rect!(float) rect_c,
          LayoutRect rect_d,
          TypedTransform2D!(float, UnknownUnit, LayoutUnit) transform_a,
          TypedTransform2D!(float, LayoutUnit, UnknownUnit) transform_b);

}  // extern(C)
