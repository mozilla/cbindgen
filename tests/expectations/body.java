import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class MyCLikeEnum extends IntegerType {
    public MyCLikeEnum() {
      super(4);
    }

    public MyCLikeEnum(long value) {
      super(4, value);
    }

    public MyCLikeEnum(Pointer p) {
      this(p.getInt(0));
    }
    public static final MyCLikeEnum Foo1 = new MyCLikeEnum(1);
    public static final MyCLikeEnum Bar1 = new MyCLikeEnum(2);
    public static final MyCLikeEnum Baz1 = new MyCLikeEnum(3);

  }

  class MyCLikeEnumByReference extends ByReference {
    public MyCLikeEnumByReference() {
      super(4);
    }

    public MyCLikeEnumByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public MyCLikeEnum getValue() {
      return new MyCLikeEnum(getPointer().getInt(0));
    }

    public void setValue(MyCLikeEnum value) {
      getPointer().setInt(0, value.intValue());
    }

  }

  class MyCLikeEnum_Prepended extends IntegerType {
    public MyCLikeEnum_Prepended() {
      super(4);
    }

    public MyCLikeEnum_Prepended(long value) {
      super(4, value);
    }

    public MyCLikeEnum_Prepended(Pointer p) {
      this(p.getInt(0));
    }
    public static final MyCLikeEnum_Prepended Foo1_Prepended = new MyCLikeEnum_Prepended(1);
    public static final MyCLikeEnum_Prepended Bar1_Prepended = new MyCLikeEnum_Prepended(2);
    public static final MyCLikeEnum_Prepended Baz1_Prepended = new MyCLikeEnum_Prepended(3);

  }

  class MyCLikeEnum_PrependedByReference extends ByReference {
    public MyCLikeEnum_PrependedByReference() {
      super(4);
    }

    public MyCLikeEnum_PrependedByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public MyCLikeEnum_Prepended getValue() {
      return new MyCLikeEnum_Prepended(getPointer().getInt(0));
    }

    public void setValue(MyCLikeEnum_Prepended value) {
      getPointer().setInt(0, value.intValue());
    }

  }


  @Structure.FieldOrder({"i"})
  class MyFancyStruct extends Structure implements Structure.ByValue {
    public MyFancyStruct() {
      super();
    }

    public MyFancyStruct(Pointer p) {
      super(p);
    }

    public int i;

  }

  @Structure.FieldOrder({"i"})
  class MyFancyStructByReference extends Structure implements Structure.ByReference {
    public MyFancyStructByReference() {
      super();
    }

    public MyFancyStructByReference(Pointer p) {
      super(p);
    }

    public int i;

  }


  class MyFancyEnum extends IntegerType {
    public MyFancyEnum() {
      super(4);
    }

    public MyFancyEnum(long value) {
      super(4, value);
    }

    public MyFancyEnum(Pointer p) {
      this(p.getInt(0));
    }
    public static final MyFancyEnum Foo = new MyFancyEnum(1);
    public static final MyFancyEnum Bar = new MyFancyEnum(2);
    public static final MyFancyEnum Baz = new MyFancyEnum(3);

  }

  class MyFancyEnumByReference extends ByReference {
    public MyFancyEnumByReference() {
      super(4);
    }

    public MyFancyEnumByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public MyFancyEnum getValue() {
      return new MyFancyEnum(getPointer().getInt(0));
    }

    public void setValue(MyFancyEnum value) {
      getPointer().setInt(0, value.intValue());
    }

  }


  @Structure.FieldOrder({"f", "u"})
  class MyUnion extends Union implements Structure.ByValue {
    public MyUnion() {
      super();
    }

    public MyUnion(Pointer p) {
      super(p);
    }

    public float f;
    public int u;

  }

  @Structure.FieldOrder({"f", "u"})
  class MyUnionByReference extends Union implements Structure.ByReference {
    public MyUnionByReference() {
      super();
    }

    public MyUnionByReference(Pointer p) {
      super(p);
    }

    public float f;
    public int u;

  }



  @Structure.FieldOrder({"i"})
  class MyFancyStruct_Prepended extends Structure implements Structure.ByValue {
    public MyFancyStruct_Prepended() {
      super();
    }

    public MyFancyStruct_Prepended(Pointer p) {
      super(p);
    }

    public int i;

  }

  @Structure.FieldOrder({"i"})
  class MyFancyStruct_PrependedByReference extends Structure implements Structure.ByReference {
    public MyFancyStruct_PrependedByReference() {
      super();
    }

    public MyFancyStruct_PrependedByReference(Pointer p) {
      super(p);
    }

    public int i;

  }


  class MyFancyEnum_Prepended extends IntegerType {
    public MyFancyEnum_Prepended() {
      super(4);
    }

    public MyFancyEnum_Prepended(long value) {
      super(4, value);
    }

    public MyFancyEnum_Prepended(Pointer p) {
      this(p.getInt(0));
    }
    public static final MyFancyEnum_Prepended Foo_Prepended = new MyFancyEnum_Prepended(1);
    public static final MyFancyEnum_Prepended Bar_Prepended = new MyFancyEnum_Prepended(2);
    public static final MyFancyEnum_Prepended Baz_Prepended = new MyFancyEnum_Prepended(3);

  }

  class MyFancyEnum_PrependedByReference extends ByReference {
    public MyFancyEnum_PrependedByReference() {
      super(4);
    }

    public MyFancyEnum_PrependedByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public MyFancyEnum_Prepended getValue() {
      return new MyFancyEnum_Prepended(getPointer().getInt(0));
    }

    public void setValue(MyFancyEnum_Prepended value) {
      getPointer().setInt(0, value.intValue());
    }

  }


  @Structure.FieldOrder({"f", "u"})
  class MyUnion_Prepended extends Union implements Structure.ByValue {
    public MyUnion_Prepended() {
      super();
    }

    public MyUnion_Prepended(Pointer p) {
      super(p);
    }

    public float f;
    public int u;

  }

  @Structure.FieldOrder({"f", "u"})
  class MyUnion_PrependedByReference extends Union implements Structure.ByReference {
    public MyUnion_PrependedByReference() {
      super();
    }

    public MyUnion_PrependedByReference(Pointer p) {
      super(p);
    }

    public float f;
    public int u;

  }


  void root(MyFancyStruct s, 
            MyFancyEnum e, 
            MyCLikeEnum c, 
            MyUnion u, 
            MyFancyStruct_Prepended sp, 
            MyFancyEnum_Prepended ep, 
            MyCLikeEnum_Prepended cp, 
            MyUnion_Prepended up);

}