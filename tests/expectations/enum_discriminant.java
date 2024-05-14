import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  /* Unsupported literal for constant FOURTY_FOUR */



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
    public static final E A = new E(1);
    public static final E B = new E(2);
    public static final E C = new E(3);
    public static final E D = new E(4);
    public static final E F = new E(5);
    public static final E G = new E(6);
    public static final E H = new E(7);

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


  void root(EByReference arg0);

}