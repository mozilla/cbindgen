import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  /* Unsupported literal for constant C_H */


  class C_E extends IntegerType {
    public C_E() {
      super(4);
    }

    public C_E(long value) {
      super(4, value);
    }

    public C_E(Pointer p) {
      this(p.getInt(0));
    }
    public static final C_E x = new C_E(0);
    public static final C_E y = new C_E(1);

  }

  class C_EByReference extends ByReference {
    public C_EByReference() {
      super(4);
    }

    public C_EByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public C_E getValue() {
      return new C_E(getPointer().getInt(0));
    }

    public void setValue(C_E value) {
      getPointer().setInt(0, value.intValue());
    }

  }

  class C_A extends PointerType {
    public C_A() {
      super(null);
    }
    public C_A(Pointer p) {
      super(p);
    }
  }

  class C_AByReference extends C_A {
    public C_AByReference() {
      super(null);
    }
    public C_AByReference(Pointer p) {
      super(p);
    }
  }

  class C_C extends PointerType {
    public C_C() {
      super(null);
    }
    public C_C(Pointer p) {
      super(p);
    }
  }

  class C_CByReference extends C_C {
    public C_CByReference() {
      super(null);
    }
    public C_CByReference(Pointer p) {
      super(p);
    }
  }


  @Structure.FieldOrder({"x", "y"})
  class C_AwesomeB extends Structure implements Structure.ByValue {
    public C_AwesomeB() {
      super();
    }

    public C_AwesomeB(Pointer p) {
      super(p);
    }

    public int x;
    public float y;

  }

  @Structure.FieldOrder({"x", "y"})
  class C_AwesomeBByReference extends Structure implements Structure.ByReference {
    public C_AwesomeBByReference() {
      super();
    }

    public C_AwesomeBByReference(Pointer p) {
      super(p);
    }

    public int x;
    public float y;

  }



  @Structure.FieldOrder({"x", "y"})
  class C_D extends Union implements Structure.ByValue {
    public C_D() {
      super();
    }

    public C_D(Pointer p) {
      super(p);
    }

    public int x;
    public float y;

  }

  @Structure.FieldOrder({"x", "y"})
  class C_DByReference extends Union implements Structure.ByReference {
    public C_DByReference() {
      super();
    }

    public C_DByReference(Pointer p) {
      super(p);
    }

    public int x;
    public float y;

  }


  class C_F extends C_A {
    public C_F() {
      super();
    }
    public C_F(Pointer p) {
      super(p);
    }
  }

  class C_FByReference extends C_AByReference {
    public C_FByReference() {
      super();
    }
    public C_FByReference(Pointer p) {
      super(p);
    }
  }

  /* Unsupported literal for constant C_I */


  /* Not implemented yet : Static { path: Path { name: "G" }, export_name: "G", ty: Primitive(Integer { zeroable: true, signed: true, kind: B32 }), mutable: false, cfg: None, annotations: AnnotationSet { annotations: {}, must_use: false, deprecated: None }, documentation: Documentation { doc_comment: [] } } */

  void root(C_AByReference a, C_AwesomeB b, C_C c, C_D d, C_E e, C_F f);

}