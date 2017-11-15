#include <cstdint>
#include <cstdlib>

struct LayoutUnit;

struct UnknownUnit;

template<typename T, typename Unit>
struct TypedLength {
  T _0;
};

template<typename T>
struct Length {
  T _0;
};

typedef TypedLength<float, LayoutUnit> LayoutLength;

template<typename T, typename U>
struct TypedSideOffsets2D {
  T top;
  T right;
  T bottom;
  T left;
};

template<typename T>
struct SideOffsets2D {
  T top;
  T right;
  T bottom;
  T left;
};

typedef TypedSideOffsets2D<float, LayoutUnit> LayoutSideOffsets2D;

template<typename T, typename U>
struct TypedSize2D {
  T width;
  T height;
};

template<typename T>
struct Size2D {
  T width;
  T height;
};

typedef TypedSize2D<float, LayoutUnit> LayoutSize2D;

template<typename T, typename U>
struct TypedPoint2D {
  T x;
  T y;
};

template<typename T>
struct Point2D {
  T x;
  T y;
};

typedef TypedPoint2D<float, LayoutUnit> LayoutPoint2D;

template<typename T, typename U>
struct TypedRect {
  TypedPoint2D<T, U> origin;
  TypedSize2D<T, U> size;
};

template<typename T>
struct Rect {
  TypedPoint2D<T, UnknownUnit> origin;
  TypedSize2D<T, UnknownUnit> size;
};

typedef TypedRect<float, LayoutUnit> LayoutRect;

template<typename T, typename Src, typename Dst>
struct TypedTransform2D {
  T m11;
  T m12;
  T m21;
  T m22;
  T m31;
  T m32;
};

extern "C" {

void root(TypedLength<float, UnknownUnit> length_a,
          TypedLength<float, LayoutUnit> length_b,
          Length<float> length_c,
          LayoutLength length_d,
          TypedSideOffsets2D<float, UnknownUnit> side_offsets_a,
          TypedSideOffsets2D<float, LayoutUnit> side_offsets_b,
          SideOffsets2D<float> side_offsets_c,
          LayoutSideOffsets2D side_offsets_d,
          TypedSize2D<float, UnknownUnit> size_a,
          TypedSize2D<float, LayoutUnit> size_b,
          Size2D<float> size_c,
          LayoutSize2D size_d,
          TypedPoint2D<float, UnknownUnit> point_a,
          TypedPoint2D<float, LayoutUnit> point_b,
          Point2D<float> point_c,
          LayoutPoint2D point_d,
          TypedRect<float, UnknownUnit> rect_a,
          TypedRect<float, LayoutUnit> rect_b,
          Rect<float> rect_c,
          LayoutRect rect_d,
          TypedTransform2D<float, UnknownUnit, LayoutUnit> transform_a,
          TypedTransform2D<float, LayoutUnit, UnknownUnit> transform_b);

} // extern "C"
