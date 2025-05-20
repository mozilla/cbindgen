import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"namespace_", "float_"})
  class A extends Structure implements Structure.ByValue {
    public A() {
      super();
    }

    public A(Pointer p) {
      super(p);
    }

    public int namespace_;
    public float float_;

  }

  @Structure.FieldOrder({"namespace_", "float_"})
  class AByReference extends Structure implements Structure.ByReference {
    public AByReference() {
      super();
    }

    public AByReference(Pointer p) {
      super(p);
    }

    public int namespace_;
    public float float_;

  }



  @Structure.FieldOrder({"namespace_", "float_"})
  class B extends Structure implements Structure.ByValue {
    public B() {
      super();
    }

    public B(Pointer p) {
      super(p);
    }

    public int namespace_;
    public float float_;

  }

  @Structure.FieldOrder({"namespace_", "float_"})
  class BByReference extends Structure implements Structure.ByReference {
    public BByReference() {
      super();
    }

    public BByReference(Pointer p) {
      super(p);
    }

    public int namespace_;
    public float float_;

  }



  class C extends IntegerType {
    public C() {
      super(4, true);
    }

    public C(long value) {
      super(4, value, true);
    }

    public C(Pointer p) {
      this(p.getInt(0));
    }
    public static final C D = new C(1);

  }

  class CByReference extends ByReference {
    public CByReference() {
      super(4);
    }

    public CByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public C getValue() {
      Pointer p = getPointer();
      return new C(p.getInt(0));
    }

    public void setValue(C value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class E extends IntegerType {
    public E() {
      super(4, true);
    }

    public E(long value) {
      super(4, value, true);
    }

    public E(Pointer p) {
      this(p.getInt(0));
    }
    public static final E Double = new E(1);
    public static final E Float = new E(2);

  }

  class EByReference extends ByReference {
    public EByReference() {
      super(4);
    }

    public EByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public E getValue() {
      Pointer p = getPointer();
      return new E(p.getInt(0));
    }

    public void setValue(E value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class F extends IntegerType {
    public F() {
      super(4, true);
    }

    public F(long value) {
      super(4, value, true);
    }

    public F(Pointer p) {
      this(p.getInt(0));
    }
    public static final F double_ = new F(1);
    public static final F float_ = new F(2);

  }

  class FByReference extends ByReference {
    public FByReference() {
      super(4);
    }

    public FByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public F getValue() {
      Pointer p = getPointer();
      return new F(p.getInt(0));
    }

    public void setValue(F value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }


  void root(A a, B b, C c, E e, F f, int namespace_, float float_);

}