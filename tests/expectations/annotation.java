import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class C extends IntegerType {
    public C() {
      super(4);
    }

    public C(long value) {
      super(4, value);
    }

    public C(Pointer p) {
      this(p.getInt(0));
    }
    public static final C X = new C(2);
    public static final C Y = new C(3);

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
      return new C(getPointer().getInt(0));
    }

    public void setValue(C value) {
      getPointer().setInt(0, value.intValue());
    }

  }


  @Structure.FieldOrder({"m0"})
  class A extends Structure implements Structure.ByValue {
    public A() {
      super();
    }

    public A(Pointer p) {
      super(p);
    }

    public int m0;

  }

  @Structure.FieldOrder({"m0"})
  class AByReference extends Structure implements Structure.ByReference {
    public AByReference() {
      super();
    }

    public AByReference(Pointer p) {
      super(p);
    }

    public int m0;

  }



  @Structure.FieldOrder({"x", "y"})
  class B extends Structure implements Structure.ByValue {
    public B() {
      super();
    }

    public B(Pointer p) {
      super(p);
    }

    public int x;
    public float y;

  }

  @Structure.FieldOrder({"x", "y"})
  class BByReference extends Structure implements Structure.ByReference {
    public BByReference() {
      super();
    }

    public BByReference(Pointer p) {
      super(p);
    }

    public int x;
    public float y;

  }


  class F extends IntegerType {
    public F() {
      super(4);
    }

    public F(long value) {
      super(4, value);
    }

    public F(Pointer p) {
      this(p.getInt(0));
    }
    public static final F Foo = new F(1);
    public static final F Bar = new F(2);
    public static final F Baz = new F(3);

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
      return new F(getPointer().getInt(0));
    }

    public void setValue(F value) {
      getPointer().setInt(0, value.intValue());
    }

  }

  class H extends IntegerType {
    public H() {
      super(4);
    }

    public H(long value) {
      super(4, value);
    }

    public H(Pointer p) {
      this(p.getInt(0));
    }
    public static final H Hello = new H(1);
    public static final H There = new H(2);
    public static final H Everyone = new H(3);

  }

  class HByReference extends ByReference {
    public HByReference() {
      super(4);
    }

    public HByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public H getValue() {
      return new H(getPointer().getInt(0));
    }

    public void setValue(H value) {
      getPointer().setInt(0, value.intValue());
    }

  }

  void root(A x, B y, C z, F f, H h);

}