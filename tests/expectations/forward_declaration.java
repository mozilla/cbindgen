
import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"fields", "num_fields"})
  class StructInfo extends Structure implements Structure.ByValue {
    public StructInfo() {
      super();
    }

    public StructInfo(Pointer p) {
      super(p);
    }

    public PointerByReference fields;
    public _Size num_fields;

  }

  @Structure.FieldOrder({"fields", "num_fields"})
  class StructInfoByReference extends Structure implements Structure.ByReference {
    public StructInfoByReference() {
      super();
    }

    public StructInfoByReference(Pointer p) {
      super(p);
    }

    public PointerByReference fields;
    public _Size num_fields;

  }



  class TypeData extends IntegerType {
    public TypeData() {
      super(4, true);
    }

    public TypeData(long value) {
      super(4, value, true);
    }

    public TypeData(Pointer p) {
      this(p.getInt(0));
    }
    public static final TypeData Primitive = new TypeData(1);
    public static final TypeData Struct = new TypeData(2);

  }

  class TypeDataByReference extends ByReference {
    public TypeDataByReference() {
      super(4);
    }

    public TypeDataByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public TypeData getValue() {
      Pointer p = getPointer();
      return new TypeData(p.getInt(0));
    }

    public void setValue(TypeData value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  @Structure.FieldOrder({"data"})
  class TypeInfo extends Structure implements Structure.ByValue {
    public TypeInfo() {
      super();
    }

    public TypeInfo(Pointer p) {
      super(p);
    }

    public TypeData data;

  }

  @Structure.FieldOrder({"data"})
  class TypeInfoByReference extends Structure implements Structure.ByReference {
    public TypeInfoByReference() {
      super();
    }

    public TypeInfoByReference(Pointer p) {
      super(p);
    }

    public TypeData data;

  }


  void root(TypeInfo x);

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

