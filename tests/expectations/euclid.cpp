#include <cstdint>
#include <cstdlib>

extern "C" {

struct LayoutUnit;

struct UnknownUnit;

struct TypedLength_f32__UnknownUnit {
  float _0;
};

struct TypedLength_f32__LayoutUnit {
  float _0;
};

struct Length_f32 {
  float _0;
};

typedef TypedLength_f32__LayoutUnit LayoutLength;

struct TypedSideOffsets2D_f32__UnknownUnit {
  float top;
  float right;
  float bottom;
  float left;
};

struct TypedSideOffsets2D_f32__LayoutUnit {
  float top;
  float right;
  float bottom;
  float left;
};

struct SideOffsets2D_f32 {
  float top;
  float right;
  float bottom;
  float left;
};

typedef TypedSideOffsets2D_f32__LayoutUnit LayoutSideOffsets2D;

struct TypedSize2D_f32__UnknownUnit {
  float width;
  float height;
};

struct TypedSize2D_f32__LayoutUnit {
  float width;
  float height;
};

struct Size2D_f32 {
  float width;
  float height;
};

typedef TypedSize2D_f32__LayoutUnit LayoutSize2D;

struct TypedPoint2D_f32__UnknownUnit {
  float x;
  float y;
};

struct TypedPoint2D_f32__LayoutUnit {
  float x;
  float y;
};

struct Point2D_f32 {
  float x;
  float y;
};

typedef TypedPoint2D_f32__LayoutUnit LayoutPoint2D;

struct TypedRect_f32__UnknownUnit {
  TypedPoint2D_f32__UnknownUnit origin;
  TypedSize2D_f32__UnknownUnit size;
};

struct TypedRect_f32__LayoutUnit {
  TypedPoint2D_f32__LayoutUnit origin;
  TypedSize2D_f32__LayoutUnit size;
};

struct Rect_f32 {
  TypedPoint2D_f32__UnknownUnit origin;
  TypedSize2D_f32__UnknownUnit size;
};

typedef TypedRect_f32__LayoutUnit LayoutRect;

struct TypedTransform2D_f32__UnknownUnit__LayoutUnit {
  float m11;
  float m12;
  float m21;
  float m22;
  float m31;
  float m32;
};

struct TypedTransform2D_f32__LayoutUnit__UnknownUnit {
  float m11;
  float m12;
  float m21;
  float m22;
  float m31;
  float m32;
};

void root(TypedLength_f32__UnknownUnit length_a,
          TypedLength_f32__LayoutUnit length_b,
          Length_f32 length_c,
          LayoutLength length_d,
          TypedSideOffsets2D_f32__UnknownUnit side_offsets_a,
          TypedSideOffsets2D_f32__LayoutUnit side_offsets_b,
          SideOffsets2D_f32 side_offsets_c,
          LayoutSideOffsets2D side_offsets_d,
          TypedSize2D_f32__UnknownUnit size_a,
          TypedSize2D_f32__LayoutUnit size_b,
          Size2D_f32 size_c,
          LayoutSize2D size_d,
          TypedPoint2D_f32__UnknownUnit point_a,
          TypedPoint2D_f32__LayoutUnit point_b,
          Point2D_f32 point_c,
          LayoutPoint2D point_d,
          TypedRect_f32__UnknownUnit rect_a,
          TypedRect_f32__LayoutUnit rect_b,
          Rect_f32 rect_c,
          LayoutRect rect_d,
          TypedTransform2D_f32__UnknownUnit__LayoutUnit transform_a,
          TypedTransform2D_f32__LayoutUnit__UnknownUnit transform_b);

} // extern "C"

template<typename T>
struct Length;

template<>
struct Length<float> : public Length_f32 {

};

template<typename T>
struct Point2D;

template<>
struct Point2D<float> : public Point2D_f32 {

};

template<typename T>
struct Rect;

template<>
struct Rect<float> : public Rect_f32 {

};

template<typename T>
struct SideOffsets2D;

template<>
struct SideOffsets2D<float> : public SideOffsets2D_f32 {

};

template<typename T>
struct Size2D;

template<>
struct Size2D<float> : public Size2D_f32 {

};

template<typename T, typename Unit>
struct TypedLength;

template<>
struct TypedLength<float, LayoutUnit> : public TypedLength_f32__LayoutUnit {

};

template<>
struct TypedLength<float, UnknownUnit> : public TypedLength_f32__UnknownUnit {

};

template<typename T, typename U>
struct TypedPoint2D;

template<>
struct TypedPoint2D<float, LayoutUnit> : public TypedPoint2D_f32__LayoutUnit {

};

template<>
struct TypedPoint2D<float, UnknownUnit> : public TypedPoint2D_f32__UnknownUnit {

};

template<typename T, typename U>
struct TypedRect;

template<>
struct TypedRect<float, LayoutUnit> : public TypedRect_f32__LayoutUnit {

};

template<>
struct TypedRect<float, UnknownUnit> : public TypedRect_f32__UnknownUnit {

};

template<typename T, typename U>
struct TypedSideOffsets2D;

template<>
struct TypedSideOffsets2D<float, LayoutUnit> : public TypedSideOffsets2D_f32__LayoutUnit {

};

template<>
struct TypedSideOffsets2D<float, UnknownUnit> : public TypedSideOffsets2D_f32__UnknownUnit {

};

template<typename T, typename U>
struct TypedSize2D;

template<>
struct TypedSize2D<float, LayoutUnit> : public TypedSize2D_f32__LayoutUnit {

};

template<>
struct TypedSize2D<float, UnknownUnit> : public TypedSize2D_f32__UnknownUnit {

};

template<typename T, typename Src, typename Dst>
struct TypedTransform2D;

template<>
struct TypedTransform2D<float, UnknownUnit, LayoutUnit> : public TypedTransform2D_f32__UnknownUnit__LayoutUnit {

};

template<>
struct TypedTransform2D<float, LayoutUnit, UnknownUnit> : public TypedTransform2D_f32__LayoutUnit__UnknownUnit {

};
