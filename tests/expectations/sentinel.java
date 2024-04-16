import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  class A extends IntegerType {
    public A() {
      super(4, true);
    }

    public A(long value) {
      super(4, value, true);
    }

    public A(Pointer p) {
      this(p.getInt(0));
    }
    public static final A A_A1 = new A(1);
    public static final A A_A2 = new A(2);
    public static final A A_A3 = new A(3);

    /**
     * Must be last for serialization purposes
     */
    public static final A A_Sentinel = new A(4);

  }

  class AByReference extends ByReference {
    public AByReference() {
      super(4);
    }

    public AByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public A getValue() {
      Pointer p = getPointer();
      return new A(p.getInt(0));
    }

    public void setValue(A value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class B extends IntegerType {
    public B() {
      super(4, true);
    }

    public B(long value) {
      super(4, value, true);
    }

    public B(Pointer p) {
      this(p.getInt(0));
    }
    public static final B B_B1 = new B(1);
    public static final B B_B2 = new B(2);
    public static final B B_B3 = new B(3);

    /**
     * Must be last for serialization purposes
     */
    public static final B B_Sentinel = new B(4);

  }

  class BByReference extends ByReference {
    public BByReference() {
      super(4);
    }

    public BByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public B getValue() {
      Pointer p = getPointer();
      return new B(p.getInt(0));
    }

    public void setValue(B value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

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
    public static final C C_C1 = new C(1);
    public static final C C_C2 = new C(2);
    public static final C C_C3 = new C(3);

    /**
     * Must be last for serialization purposes
     */
    public static final C C_Sentinel = new C(4);

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


  void root(A a, B b, C c);

}