import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class Enum extends IntegerType {
    public Enum() {
      super(4);
    }

    public Enum(long value) {
      super(4, value);
    }

    public Enum(Pointer p) {
      this(p.getInt(0));
    }
    public static final Enum a = new Enum(1);
    public static final Enum b = new Enum(2);

  }

  class EnumByReference extends ByReference {
    public EnumByReference() {
      super(4);
    }

    public EnumByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public Enum getValue() {
      return new Enum(getPointer().getInt(0));
    }

    public void setValue(Enum value) {
      getPointer().setInt(0, value.intValue());
    }

  }


  @Structure.FieldOrder({"field"})
  class Struct extends Structure implements Structure.ByValue {
    public Struct() {
      super();
    }

    public Struct(Pointer p) {
      super(p);
    }

    public Enum field;

  }

  @Structure.FieldOrder({"field"})
  class StructByReference extends Structure implements Structure.ByReference {
    public StructByReference() {
      super();
    }

    public StructByReference(Pointer p) {
      super(p);
    }

    public Enum field;

  }


  /* Not implemented yet : Static { path: Path { name: "STATIC" }, export_name: "STATIC", ty: Path(GenericPath { path: Path { name: "Enum" }, export_name: "Enum", generics: [], ctype: None }), mutable: false, cfg: None, annotations: AnnotationSet { annotations: {}, must_use: false, deprecated: None }, documentation: Documentation { doc_comment: [] } } */

  void fn(Struct arg);

}