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
      super(4, true);
    }

    public C(long value) {
      super(4, value, true);
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
      Pointer p = getPointer();
      return new C(p.getInt(0));
    }

    public void setValue(C value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  @Structure.FieldOrder({"_0"})
  class A extends Structure implements Structure.ByValue {
    public A() {
      super();
    }

    public A(Pointer p) {
      super(p);
    }

    public int _0;

  }

  @Structure.FieldOrder({"_0"})
  class AByReference extends Structure implements Structure.ByReference {
    public AByReference() {
      super();
    }

    public AByReference(Pointer p) {
      super(p);
    }

    public int _0;

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



  @Structure.FieldOrder({"List", "Of", "Things"})
  class D extends Structure implements Structure.ByValue {
    public D() {
      super();
    }

    public D(Pointer p) {
      super(p);
    }

    public byte List;
    public _Size Of;
    public B Things;

  }

  @Structure.FieldOrder({"List", "Of", "Things"})
  class DByReference extends Structure implements Structure.ByReference {
    public DByReference() {
      super();
    }

    public DByReference(Pointer p) {
      super(p);
    }

    public byte List;
    public _Size Of;
    public B Things;

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
      Pointer p = getPointer();
      return new F(p.getInt(0));
    }

    public void setValue(F value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class H extends IntegerType {
    public H() {
      super(4, true);
    }

    public H(long value) {
      super(4, value, true);
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
      Pointer p = getPointer();
      return new H(p.getInt(0));
    }

    public void setValue(H value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class I extends IntegerType {
    public I() {
      super(4, true);
    }

    public I(long value) {
      super(4, value, true);
    }

    public I(Pointer p) {
      this(p.getInt(0));
    }
    public static final I ThereAgain = new I(1);
    public static final I SomethingElse = new I(2);

  }

  class IByReference extends ByReference {
    public IByReference() {
      super(4);
    }

    public IByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public I getValue() {
      Pointer p = getPointer();
      return new I(p.getInt(0));
    }

    public void setValue(I value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }


  void root(A a, B b, C c, D d, F f, H h, I i);

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