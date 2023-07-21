
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
    public NativeLong num_fields;

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
    public NativeLong num_fields;

  }


  class TypeData extends IntegerType {
    public TypeData() {
      super(4);
    }

    public TypeData(long value) {
      super(4, value);
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
      return new TypeData(getPointer().getInt(0));
    }

    public void setValue(TypeData value) {
      getPointer().setInt(0, value.intValue());
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

}

