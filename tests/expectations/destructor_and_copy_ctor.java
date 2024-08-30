
import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  class FillRule extends IntegerType {
    public FillRule() {
      super(4, true);
    }

    public FillRule(long value) {
      super(4, value, true);
    }

    public FillRule(Pointer p) {
      this(p.getInt(0));
    }
    public static final FillRule A = new FillRule(1);
    public static final FillRule B = new FillRule(2);

  }

  class FillRuleByReference extends ByReference {
    public FillRuleByReference() {
      super(4);
    }

    public FillRuleByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public FillRule getValue() {
      Pointer p = getPointer();
      return new FillRule(p.getInt(0));
    }

    public void setValue(FillRule value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }




  /**
   * This will have a destructor manually implemented via variant_body, and
   * similarly a Drop impl in Rust.
   */
  @Structure.FieldOrder({"len", "ptr"})
  class OwnedSlice_u32 extends Structure implements Structure.ByValue {
    public OwnedSlice_u32() {
      super();
    }

    public OwnedSlice_u32(Pointer p) {
      super(p);
    }

    public _Size len;
    public IntByReference ptr;

  }


  /**
   * This will have a destructor manually implemented via variant_body, and
   * similarly a Drop impl in Rust.
   */
  @Structure.FieldOrder({"len", "ptr"})
  class OwnedSlice_u32ByReference extends Structure implements Structure.ByReference {
    public OwnedSlice_u32ByReference() {
      super();
    }

    public OwnedSlice_u32ByReference(Pointer p) {
      super(p);
    }

    public _Size len;
    public IntByReference ptr;

  }



  @Structure.FieldOrder({"fill", "coordinates"})
  class Polygon_u32 extends Structure implements Structure.ByValue {
    public Polygon_u32() {
      super();
    }

    public Polygon_u32(Pointer p) {
      super(p);
    }

    public FillRule fill;
    public OwnedSlice_u32 coordinates;

  }

  @Structure.FieldOrder({"fill", "coordinates"})
  class Polygon_u32ByReference extends Structure implements Structure.ByReference {
    public Polygon_u32ByReference() {
      super();
    }

    public Polygon_u32ByReference(Pointer p) {
      super(p);
    }

    public FillRule fill;
    public OwnedSlice_u32 coordinates;

  }




  /**
   * This will have a destructor manually implemented via variant_body, and
   * similarly a Drop impl in Rust.
   */
  @Structure.FieldOrder({"len", "ptr"})
  class OwnedSlice_i32 extends Structure implements Structure.ByValue {
    public OwnedSlice_i32() {
      super();
    }

    public OwnedSlice_i32(Pointer p) {
      super(p);
    }

    public _Size len;
    public IntByReference ptr;

  }


  /**
   * This will have a destructor manually implemented via variant_body, and
   * similarly a Drop impl in Rust.
   */
  @Structure.FieldOrder({"len", "ptr"})
  class OwnedSlice_i32ByReference extends Structure implements Structure.ByReference {
    public OwnedSlice_i32ByReference() {
      super();
    }

    public OwnedSlice_i32ByReference(Pointer p) {
      super(p);
    }

    public _Size len;
    public IntByReference ptr;

  }



  class Foo_u32 extends IntegerType {
    public Foo_u32() {
      super(4, true);
    }

    public Foo_u32(long value) {
      super(4, value, true);
    }

    public Foo_u32(Pointer p) {
      this(p.getInt(0));
    }
    public static final Foo_u32 Bar_u32 = new Foo_u32(1);
    public static final Foo_u32 Polygon1_u32 = new Foo_u32(2);
    public static final Foo_u32 Slice1_u32 = new Foo_u32(3);
    public static final Foo_u32 Slice2_u32 = new Foo_u32(4);
    public static final Foo_u32 Slice3_u32 = new Foo_u32(5);
    public static final Foo_u32 Slice4_u32 = new Foo_u32(6);

  }

  class Foo_u32ByReference extends ByReference {
    public Foo_u32ByReference() {
      super(4);
    }

    public Foo_u32ByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public Foo_u32 getValue() {
      Pointer p = getPointer();
      return new Foo_u32(p.getInt(0));
    }

    public void setValue(Foo_u32 value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  @Structure.FieldOrder({"fill", "coordinates"})
  class Polygon_i32 extends Structure implements Structure.ByValue {
    public Polygon_i32() {
      super();
    }

    public Polygon_i32(Pointer p) {
      super(p);
    }

    public FillRule fill;
    public OwnedSlice_i32 coordinates;

  }

  @Structure.FieldOrder({"fill", "coordinates"})
  class Polygon_i32ByReference extends Structure implements Structure.ByReference {
    public Polygon_i32ByReference() {
      super();
    }

    public Polygon_i32ByReference(Pointer p) {
      super(p);
    }

    public FillRule fill;
    public OwnedSlice_i32 coordinates;

  }



  class Baz_i32 extends IntegerType {
    public Baz_i32() {
      super(4, true);
    }

    public Baz_i32(long value) {
      super(4, value, true);
    }

    public Baz_i32(Pointer p) {
      this(p.getInt(0));
    }
    public static final Baz_i32 Bar2_i32 = new Baz_i32(1);
    public static final Baz_i32 Polygon21_i32 = new Baz_i32(2);
    public static final Baz_i32 Slice21_i32 = new Baz_i32(3);
    public static final Baz_i32 Slice22_i32 = new Baz_i32(4);
    public static final Baz_i32 Slice23_i32 = new Baz_i32(5);
    public static final Baz_i32 Slice24_i32 = new Baz_i32(6);

  }

  class Baz_i32ByReference extends ByReference {
    public Baz_i32ByReference() {
      super(4);
    }

    public Baz_i32ByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public Baz_i32 getValue() {
      Pointer p = getPointer();
      return new Baz_i32(p.getInt(0));
    }

    public void setValue(Baz_i32 value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class Taz extends IntegerType {
    public Taz() {
      super(4, true);
    }

    public Taz(long value) {
      super(4, value, true);
    }

    public Taz(Pointer p) {
      this(p.getInt(0));
    }
    public static final Taz Bar3 = new Taz(1);
    public static final Taz Taz1 = new Taz(2);
    public static final Taz Taz3 = new Taz(3);

  }

  class TazByReference extends ByReference {
    public TazByReference() {
      super(4);
    }

    public TazByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public Taz getValue() {
      Pointer p = getPointer();
      return new Taz(p.getInt(0));
    }

    public void setValue(Taz value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class Tazz extends IntegerType {
    public Tazz() {
      super(4, true);
    }

    public Tazz(long value) {
      super(4, value, true);
    }

    public Tazz(Pointer p) {
      this(p.getInt(0));
    }
    public static final Tazz Bar4 = new Tazz(1);
    public static final Tazz Taz2 = new Tazz(2);

  }

  class TazzByReference extends ByReference {
    public TazzByReference() {
      super(4);
    }

    public TazzByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public Tazz getValue() {
      Pointer p = getPointer();
      return new Tazz(p.getInt(0));
    }

    public void setValue(Tazz value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class Tazzz extends IntegerType {
    public Tazzz() {
      super(4, true);
    }

    public Tazzz(long value) {
      super(4, value, true);
    }

    public Tazzz(Pointer p) {
      this(p.getInt(0));
    }
    public static final Tazzz Bar5 = new Tazzz(1);
    public static final Tazzz Taz5 = new Tazzz(2);

  }

  class TazzzByReference extends ByReference {
    public TazzzByReference() {
      super(4);
    }

    public TazzzByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public Tazzz getValue() {
      Pointer p = getPointer();
      return new Tazzz(p.getInt(0));
    }

    public void setValue(Tazzz value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class Tazzzz extends IntegerType {
    public Tazzzz() {
      super(4, true);
    }

    public Tazzzz(long value) {
      super(4, value, true);
    }

    public Tazzzz(Pointer p) {
      this(p.getInt(0));
    }
    public static final Tazzzz Taz6 = new Tazzzz(1);
    public static final Tazzzz Taz7 = new Tazzzz(2);

  }

  class TazzzzByReference extends ByReference {
    public TazzzzByReference() {
      super(4);
    }

    public TazzzzByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public Tazzzz getValue() {
      Pointer p = getPointer();
      return new Tazzzz(p.getInt(0));
    }

    public void setValue(Tazzzz value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class Qux extends IntegerType {
    public Qux() {
      super(4, true);
    }

    public Qux(long value) {
      super(4, value, true);
    }

    public Qux(Pointer p) {
      this(p.getInt(0));
    }
    public static final Qux Qux1 = new Qux(1);
    public static final Qux Qux2 = new Qux(2);

  }

  class QuxByReference extends ByReference {
    public QuxByReference() {
      super(4);
    }

    public QuxByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public Qux getValue() {
      Pointer p = getPointer();
      return new Qux(p.getInt(0));
    }

    public void setValue(Qux value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }


  void root(Foo_u32ByReference a, 
            Baz_i32ByReference b, 
            TazByReference c, 
            Tazz d, 
            TazzzByReference e, 
            TazzzzByReference f, 
            QuxByReference g);

  class _Size extends IntegerType {
    public _Size() {
      super(Native.POINTER_SIZE, true);
    }

    public _Size(long value) {
      super(Native.POINTER_SIZE, value, true);
    }

    public _Size(Pointer p) {
      this(Native.POINTER_SIZE == 8 ? p.getLong(0) : p.getInt(0));
    }

  }

  class _SizeByReference extends ByReference {
    public _SizeByReference() {
      super(Native.POINTER_SIZE);
    }

    public _SizeByReference(Pointer p) {
      super(Native.POINTER_SIZE);
      setPointer(p);
    }

    public _Size getValue() {
      Pointer p = getPointer();
      return new _Size(Native.POINTER_SIZE == 8 ? p.getLong(0) : p.getInt(0));
    }

    public void setValue(_Size value) {
      Pointer p = getPointer();
      if (Native.POINTER_SIZE == 8) { p.setLong(0, value.longValue()); } else { p.setInt(0, value.intValue()); }
    }

  }

}