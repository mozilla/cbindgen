import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"data"})
  class A extends Structure implements Structure.ByValue {
    public A() {
      super();
    }

    public A(Pointer p) {
      super(p);
    }

    public IntByReference data;

  }

  @Structure.FieldOrder({"data"})
  class AByReference extends Structure implements Structure.ByReference {
    public AByReference() {
      super();
    }

    public AByReference(Pointer p) {
      super(p);
    }

    public IntByReference data;

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
    public static final E V = new E(1);
    public static final E U = new E(2);

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


  void root(A _a, E _e);

}