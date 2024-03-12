import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"x", "y"})
  class StylePoint_i32 extends Structure implements Structure.ByValue {
    public StylePoint_i32() {
      super();
    }

    public StylePoint_i32(Pointer p) {
      super(p);
    }

    public int x;
    public int y;

  }

  @Structure.FieldOrder({"x", "y"})
  class StylePoint_i32ByReference extends Structure implements Structure.ByReference {
    public StylePoint_i32ByReference() {
      super();
    }

    public StylePoint_i32ByReference(Pointer p) {
      super(p);
    }

    public int x;
    public int y;

  }



  @Structure.FieldOrder({"x", "y"})
  class StylePoint_f32 extends Structure implements Structure.ByValue {
    public StylePoint_f32() {
      super();
    }

    public StylePoint_f32(Pointer p) {
      super(p);
    }

    public float x;
    public float y;

  }

  @Structure.FieldOrder({"x", "y"})
  class StylePoint_f32ByReference extends Structure implements Structure.ByReference {
    public StylePoint_f32ByReference() {
      super();
    }

    public StylePoint_f32ByReference(Pointer p) {
      super(p);
    }

    public float x;
    public float y;

  }


  class StyleFoo_i32 extends IntegerType {
    public StyleFoo_i32() {
      super(4);
    }

    public StyleFoo_i32(long value) {
      super(4, value);
    }

    public StyleFoo_i32(Pointer p) {
      this(p.getInt(0));
    }
    public static final StyleFoo_i32 Foo_i32 = new StyleFoo_i32(1);
    public static final StyleFoo_i32 Bar_i32 = new StyleFoo_i32(2);
    public static final StyleFoo_i32 Baz_i32 = new StyleFoo_i32(3);
    public static final StyleFoo_i32 Bazz_i32 = new StyleFoo_i32(4);

  }

  class StyleFoo_i32ByReference extends ByReference {
    public StyleFoo_i32ByReference() {
      super(4);
    }

    public StyleFoo_i32ByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public StyleFoo_i32 getValue() {
      return new StyleFoo_i32(getPointer().getInt(0));
    }

    public void setValue(StyleFoo_i32 value) {
      getPointer().setInt(0, value.intValue());
    }

  }

  class StyleBar_i32 extends IntegerType {
    public StyleBar_i32() {
      super(4);
    }

    public StyleBar_i32(long value) {
      super(4, value);
    }

    public StyleBar_i32(Pointer p) {
      this(p.getInt(0));
    }
    public static final StyleBar_i32 Bar1_i32 = new StyleBar_i32(1);
    public static final StyleBar_i32 Bar2_i32 = new StyleBar_i32(2);
    public static final StyleBar_i32 Bar3_i32 = new StyleBar_i32(3);
    public static final StyleBar_i32 Bar4_i32 = new StyleBar_i32(4);

  }

  class StyleBar_i32ByReference extends ByReference {
    public StyleBar_i32ByReference() {
      super(4);
    }

    public StyleBar_i32ByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public StyleBar_i32 getValue() {
      return new StyleBar_i32(getPointer().getInt(0));
    }

    public void setValue(StyleBar_i32 value) {
      getPointer().setInt(0, value.intValue());
    }

  }


  @Structure.FieldOrder({"x", "y"})
  class StylePoint_u32 extends Structure implements Structure.ByValue {
    public StylePoint_u32() {
      super();
    }

    public StylePoint_u32(Pointer p) {
      super(p);
    }

    public int x;
    public int y;

  }

  @Structure.FieldOrder({"x", "y"})
  class StylePoint_u32ByReference extends Structure implements Structure.ByReference {
    public StylePoint_u32ByReference() {
      super();
    }

    public StylePoint_u32ByReference(Pointer p) {
      super(p);
    }

    public int x;
    public int y;

  }


  class StyleBar_u32 extends IntegerType {
    public StyleBar_u32() {
      super(4);
    }

    public StyleBar_u32(long value) {
      super(4, value);
    }

    public StyleBar_u32(Pointer p) {
      this(p.getInt(0));
    }
    public static final StyleBar_u32 Bar1_u32 = new StyleBar_u32(1);
    public static final StyleBar_u32 Bar2_u32 = new StyleBar_u32(2);
    public static final StyleBar_u32 Bar3_u32 = new StyleBar_u32(3);
    public static final StyleBar_u32 Bar4_u32 = new StyleBar_u32(4);

  }

  class StyleBar_u32ByReference extends ByReference {
    public StyleBar_u32ByReference() {
      super(4);
    }

    public StyleBar_u32ByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public StyleBar_u32 getValue() {
      return new StyleBar_u32(getPointer().getInt(0));
    }

    public void setValue(StyleBar_u32 value) {
      getPointer().setInt(0, value.intValue());
    }

  }

  class StyleBaz extends IntegerType {
    public StyleBaz() {
      super(4);
    }

    public StyleBaz(long value) {
      super(4, value);
    }

    public StyleBaz(Pointer p) {
      this(p.getInt(0));
    }
    public static final StyleBaz Baz1 = new StyleBaz(1);
    public static final StyleBaz Baz2 = new StyleBaz(2);
    public static final StyleBaz Baz3 = new StyleBaz(3);

  }

  class StyleBazByReference extends ByReference {
    public StyleBazByReference() {
      super(4);
    }

    public StyleBazByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public StyleBaz getValue() {
      return new StyleBaz(getPointer().getInt(0));
    }

    public void setValue(StyleBaz value) {
      getPointer().setInt(0, value.intValue());
    }

  }

  class StyleTaz extends IntegerType {
    public StyleTaz() {
      super(4);
    }

    public StyleTaz(long value) {
      super(4, value);
    }

    public StyleTaz(Pointer p) {
      this(p.getInt(0));
    }
    public static final StyleTaz Taz1 = new StyleTaz(1);
    public static final StyleTaz Taz2 = new StyleTaz(2);
    public static final StyleTaz Taz3 = new StyleTaz(3);

  }

  class StyleTazByReference extends ByReference {
    public StyleTazByReference() {
      super(4);
    }

    public StyleTazByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public StyleTaz getValue() {
      return new StyleTaz(getPointer().getInt(0));
    }

    public void setValue(StyleTaz value) {
      getPointer().setInt(0, value.intValue());
    }

  }

  void foo(StyleFoo_i32ByReference foo, 
           StyleBar_i32ByReference bar, 
           StyleBazByReference baz, 
           StyleTazByReference taz);

}