import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class E extends IntegerType {
    public E() {
      super(4);
    }

    public E(long value) {
      super(4, value);
    }

    public E(Pointer p) {
      this(p.getInt(0));
    }
    public static final E V = new E(1);

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
      return new E(getPointer().getInt(0));
    }

    public void setValue(E value) {
      getPointer().setInt(0, value.intValue());
    }

  }


  @Structure.FieldOrder({"field"})
  class S extends Structure implements Structure.ByValue {
    public S() {
      super();
    }

    public S(Pointer p) {
      super(p);
    }

    public byte field;

  }

  @Structure.FieldOrder({"field"})
  class SByReference extends Structure implements Structure.ByReference {
    public SByReference() {
      super();
    }

    public SByReference(Pointer p) {
      super(p);
    }

    public byte field;

  }


  class A extends IntegerType {
    public A() {
      super(1);
    }

    public A(long value) {
      super(1, value);
    }

    public A(Pointer p) {
      this(p.getByte(0));
    }

  }

  class AByReference extends ByReference {
    public AByReference() {
      super(1);
    }

    public AByReference(Pointer p) {
      super(1);
      setPointer(p);
    }

    public A getValue() {
      return new A(getPointer().getByte(0));
    }

    public void setValue(A value) {
      getPointer().setByte(0, (byte)value.intValue());
    }

  }

  /* Unsupported literal for constant C1 */


  /* Unsupported literal for constant C2 */


  public static final A C3  = new A(0);


}