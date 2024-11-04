import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"_0"})
  class TypedLength_f32__UnknownUnit extends Structure implements Structure.ByValue {
    public TypedLength_f32__UnknownUnit() {
      super();
    }

    public TypedLength_f32__UnknownUnit(Pointer p) {
      super(p);
    }

    public float _0;

  }

  @Structure.FieldOrder({"_0"})
  class TypedLength_f32__UnknownUnitByReference extends Structure implements Structure.ByReference {
    public TypedLength_f32__UnknownUnitByReference() {
      super();
    }

    public TypedLength_f32__UnknownUnitByReference(Pointer p) {
      super(p);
    }

    public float _0;

  }



  @Structure.FieldOrder({"_0"})
  class TypedLength_f32__LayoutUnit extends Structure implements Structure.ByValue {
    public TypedLength_f32__LayoutUnit() {
      super();
    }

    public TypedLength_f32__LayoutUnit(Pointer p) {
      super(p);
    }

    public float _0;

  }

  @Structure.FieldOrder({"_0"})
  class TypedLength_f32__LayoutUnitByReference extends Structure implements Structure.ByReference {
    public TypedLength_f32__LayoutUnitByReference() {
      super();
    }

    public TypedLength_f32__LayoutUnitByReference(Pointer p) {
      super(p);
    }

    public float _0;

  }


  class Length_f32 extends TypedLength_f32__UnknownUnit {
    public Length_f32() {
      super();
    }
    public Length_f32(Pointer p) {
      super(p);
    }
  }

  class Length_f32ByReference extends TypedLength_f32__UnknownUnitByReference {
    public Length_f32ByReference() {
      super();
    }
    public Length_f32ByReference(Pointer p) {
      super(p);
    }
  }

  class LayoutLength extends TypedLength_f32__LayoutUnit {
    public LayoutLength() {
      super();
    }
    public LayoutLength(Pointer p) {
      super(p);
    }
  }

  class LayoutLengthByReference extends TypedLength_f32__LayoutUnitByReference {
    public LayoutLengthByReference() {
      super();
    }
    public LayoutLengthByReference(Pointer p) {
      super(p);
    }
  }


  @Structure.FieldOrder({"top", "right", "bottom", "left"})
  class TypedSideOffsets2D_f32__UnknownUnit extends Structure implements Structure.ByValue {
    public TypedSideOffsets2D_f32__UnknownUnit() {
      super();
    }

    public TypedSideOffsets2D_f32__UnknownUnit(Pointer p) {
      super(p);
    }

    public float top;
    public float right;
    public float bottom;
    public float left;

  }

  @Structure.FieldOrder({"top", "right", "bottom", "left"})
  class TypedSideOffsets2D_f32__UnknownUnitByReference extends Structure implements Structure.ByReference {
    public TypedSideOffsets2D_f32__UnknownUnitByReference() {
      super();
    }

    public TypedSideOffsets2D_f32__UnknownUnitByReference(Pointer p) {
      super(p);
    }

    public float top;
    public float right;
    public float bottom;
    public float left;

  }



  @Structure.FieldOrder({"top", "right", "bottom", "left"})
  class TypedSideOffsets2D_f32__LayoutUnit extends Structure implements Structure.ByValue {
    public TypedSideOffsets2D_f32__LayoutUnit() {
      super();
    }

    public TypedSideOffsets2D_f32__LayoutUnit(Pointer p) {
      super(p);
    }

    public float top;
    public float right;
    public float bottom;
    public float left;

  }

  @Structure.FieldOrder({"top", "right", "bottom", "left"})
  class TypedSideOffsets2D_f32__LayoutUnitByReference extends Structure implements Structure.ByReference {
    public TypedSideOffsets2D_f32__LayoutUnitByReference() {
      super();
    }

    public TypedSideOffsets2D_f32__LayoutUnitByReference(Pointer p) {
      super(p);
    }

    public float top;
    public float right;
    public float bottom;
    public float left;

  }


  class SideOffsets2D_f32 extends TypedSideOffsets2D_f32__UnknownUnit {
    public SideOffsets2D_f32() {
      super();
    }
    public SideOffsets2D_f32(Pointer p) {
      super(p);
    }
  }

  class SideOffsets2D_f32ByReference extends TypedSideOffsets2D_f32__UnknownUnitByReference {
    public SideOffsets2D_f32ByReference() {
      super();
    }
    public SideOffsets2D_f32ByReference(Pointer p) {
      super(p);
    }
  }

  class LayoutSideOffsets2D extends TypedSideOffsets2D_f32__LayoutUnit {
    public LayoutSideOffsets2D() {
      super();
    }
    public LayoutSideOffsets2D(Pointer p) {
      super(p);
    }
  }

  class LayoutSideOffsets2DByReference extends TypedSideOffsets2D_f32__LayoutUnitByReference {
    public LayoutSideOffsets2DByReference() {
      super();
    }
    public LayoutSideOffsets2DByReference(Pointer p) {
      super(p);
    }
  }


  @Structure.FieldOrder({"width", "height"})
  class TypedSize2D_f32__UnknownUnit extends Structure implements Structure.ByValue {
    public TypedSize2D_f32__UnknownUnit() {
      super();
    }

    public TypedSize2D_f32__UnknownUnit(Pointer p) {
      super(p);
    }

    public float width;
    public float height;

  }

  @Structure.FieldOrder({"width", "height"})
  class TypedSize2D_f32__UnknownUnitByReference extends Structure implements Structure.ByReference {
    public TypedSize2D_f32__UnknownUnitByReference() {
      super();
    }

    public TypedSize2D_f32__UnknownUnitByReference(Pointer p) {
      super(p);
    }

    public float width;
    public float height;

  }



  @Structure.FieldOrder({"width", "height"})
  class TypedSize2D_f32__LayoutUnit extends Structure implements Structure.ByValue {
    public TypedSize2D_f32__LayoutUnit() {
      super();
    }

    public TypedSize2D_f32__LayoutUnit(Pointer p) {
      super(p);
    }

    public float width;
    public float height;

  }

  @Structure.FieldOrder({"width", "height"})
  class TypedSize2D_f32__LayoutUnitByReference extends Structure implements Structure.ByReference {
    public TypedSize2D_f32__LayoutUnitByReference() {
      super();
    }

    public TypedSize2D_f32__LayoutUnitByReference(Pointer p) {
      super(p);
    }

    public float width;
    public float height;

  }


  class Size2D_f32 extends TypedSize2D_f32__UnknownUnit {
    public Size2D_f32() {
      super();
    }
    public Size2D_f32(Pointer p) {
      super(p);
    }
  }

  class Size2D_f32ByReference extends TypedSize2D_f32__UnknownUnitByReference {
    public Size2D_f32ByReference() {
      super();
    }
    public Size2D_f32ByReference(Pointer p) {
      super(p);
    }
  }

  class LayoutSize2D extends TypedSize2D_f32__LayoutUnit {
    public LayoutSize2D() {
      super();
    }
    public LayoutSize2D(Pointer p) {
      super(p);
    }
  }

  class LayoutSize2DByReference extends TypedSize2D_f32__LayoutUnitByReference {
    public LayoutSize2DByReference() {
      super();
    }
    public LayoutSize2DByReference(Pointer p) {
      super(p);
    }
  }


  @Structure.FieldOrder({"x", "y"})
  class TypedPoint2D_f32__UnknownUnit extends Structure implements Structure.ByValue {
    public TypedPoint2D_f32__UnknownUnit() {
      super();
    }

    public TypedPoint2D_f32__UnknownUnit(Pointer p) {
      super(p);
    }

    public float x;
    public float y;

  }

  @Structure.FieldOrder({"x", "y"})
  class TypedPoint2D_f32__UnknownUnitByReference extends Structure implements Structure.ByReference {
    public TypedPoint2D_f32__UnknownUnitByReference() {
      super();
    }

    public TypedPoint2D_f32__UnknownUnitByReference(Pointer p) {
      super(p);
    }

    public float x;
    public float y;

  }



  @Structure.FieldOrder({"x", "y"})
  class TypedPoint2D_f32__LayoutUnit extends Structure implements Structure.ByValue {
    public TypedPoint2D_f32__LayoutUnit() {
      super();
    }

    public TypedPoint2D_f32__LayoutUnit(Pointer p) {
      super(p);
    }

    public float x;
    public float y;

  }

  @Structure.FieldOrder({"x", "y"})
  class TypedPoint2D_f32__LayoutUnitByReference extends Structure implements Structure.ByReference {
    public TypedPoint2D_f32__LayoutUnitByReference() {
      super();
    }

    public TypedPoint2D_f32__LayoutUnitByReference(Pointer p) {
      super(p);
    }

    public float x;
    public float y;

  }


  class Point2D_f32 extends TypedPoint2D_f32__UnknownUnit {
    public Point2D_f32() {
      super();
    }
    public Point2D_f32(Pointer p) {
      super(p);
    }
  }

  class Point2D_f32ByReference extends TypedPoint2D_f32__UnknownUnitByReference {
    public Point2D_f32ByReference() {
      super();
    }
    public Point2D_f32ByReference(Pointer p) {
      super(p);
    }
  }

  class LayoutPoint2D extends TypedPoint2D_f32__LayoutUnit {
    public LayoutPoint2D() {
      super();
    }
    public LayoutPoint2D(Pointer p) {
      super(p);
    }
  }

  class LayoutPoint2DByReference extends TypedPoint2D_f32__LayoutUnitByReference {
    public LayoutPoint2DByReference() {
      super();
    }
    public LayoutPoint2DByReference(Pointer p) {
      super(p);
    }
  }


  @Structure.FieldOrder({"origin", "size"})
  class TypedRect_f32__UnknownUnit extends Structure implements Structure.ByValue {
    public TypedRect_f32__UnknownUnit() {
      super();
    }

    public TypedRect_f32__UnknownUnit(Pointer p) {
      super(p);
    }

    public TypedPoint2D_f32__UnknownUnit origin;
    public TypedSize2D_f32__UnknownUnit size;

  }

  @Structure.FieldOrder({"origin", "size"})
  class TypedRect_f32__UnknownUnitByReference extends Structure implements Structure.ByReference {
    public TypedRect_f32__UnknownUnitByReference() {
      super();
    }

    public TypedRect_f32__UnknownUnitByReference(Pointer p) {
      super(p);
    }

    public TypedPoint2D_f32__UnknownUnit origin;
    public TypedSize2D_f32__UnknownUnit size;

  }



  @Structure.FieldOrder({"origin", "size"})
  class TypedRect_f32__LayoutUnit extends Structure implements Structure.ByValue {
    public TypedRect_f32__LayoutUnit() {
      super();
    }

    public TypedRect_f32__LayoutUnit(Pointer p) {
      super(p);
    }

    public TypedPoint2D_f32__LayoutUnit origin;
    public TypedSize2D_f32__LayoutUnit size;

  }

  @Structure.FieldOrder({"origin", "size"})
  class TypedRect_f32__LayoutUnitByReference extends Structure implements Structure.ByReference {
    public TypedRect_f32__LayoutUnitByReference() {
      super();
    }

    public TypedRect_f32__LayoutUnitByReference(Pointer p) {
      super(p);
    }

    public TypedPoint2D_f32__LayoutUnit origin;
    public TypedSize2D_f32__LayoutUnit size;

  }


  class Rect_f32 extends TypedRect_f32__UnknownUnit {
    public Rect_f32() {
      super();
    }
    public Rect_f32(Pointer p) {
      super(p);
    }
  }

  class Rect_f32ByReference extends TypedRect_f32__UnknownUnitByReference {
    public Rect_f32ByReference() {
      super();
    }
    public Rect_f32ByReference(Pointer p) {
      super(p);
    }
  }

  class LayoutRect extends TypedRect_f32__LayoutUnit {
    public LayoutRect() {
      super();
    }
    public LayoutRect(Pointer p) {
      super(p);
    }
  }

  class LayoutRectByReference extends TypedRect_f32__LayoutUnitByReference {
    public LayoutRectByReference() {
      super();
    }
    public LayoutRectByReference(Pointer p) {
      super(p);
    }
  }


  @Structure.FieldOrder({"m11", "m12", "m21", "m22", "m31", "m32"})
  class TypedTransform2D_f32__UnknownUnit__LayoutUnit extends Structure implements Structure.ByValue {
    public TypedTransform2D_f32__UnknownUnit__LayoutUnit() {
      super();
    }

    public TypedTransform2D_f32__UnknownUnit__LayoutUnit(Pointer p) {
      super(p);
    }

    public float m11;
    public float m12;
    public float m21;
    public float m22;
    public float m31;
    public float m32;

  }

  @Structure.FieldOrder({"m11", "m12", "m21", "m22", "m31", "m32"})
  class TypedTransform2D_f32__UnknownUnit__LayoutUnitByReference extends Structure implements Structure.ByReference {
    public TypedTransform2D_f32__UnknownUnit__LayoutUnitByReference() {
      super();
    }

    public TypedTransform2D_f32__UnknownUnit__LayoutUnitByReference(Pointer p) {
      super(p);
    }

    public float m11;
    public float m12;
    public float m21;
    public float m22;
    public float m31;
    public float m32;

  }



  @Structure.FieldOrder({"m11", "m12", "m21", "m22", "m31", "m32"})
  class TypedTransform2D_f32__LayoutUnit__UnknownUnit extends Structure implements Structure.ByValue {
    public TypedTransform2D_f32__LayoutUnit__UnknownUnit() {
      super();
    }

    public TypedTransform2D_f32__LayoutUnit__UnknownUnit(Pointer p) {
      super(p);
    }

    public float m11;
    public float m12;
    public float m21;
    public float m22;
    public float m31;
    public float m32;

  }

  @Structure.FieldOrder({"m11", "m12", "m21", "m22", "m31", "m32"})
  class TypedTransform2D_f32__LayoutUnit__UnknownUnitByReference extends Structure implements Structure.ByReference {
    public TypedTransform2D_f32__LayoutUnit__UnknownUnitByReference() {
      super();
    }

    public TypedTransform2D_f32__LayoutUnit__UnknownUnitByReference(Pointer p) {
      super(p);
    }

    public float m11;
    public float m12;
    public float m21;
    public float m22;
    public float m31;
    public float m32;

  }


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

}