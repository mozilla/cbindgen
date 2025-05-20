import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"a", "b", "c"})
  class ABC extends Structure implements Structure.ByValue {
    /* Unsupported literal for constant abc */
    /* Unsupported literal for constant bac */
    /* Unsupported literal for constant cba */
    public ABC() {
      super();
    }

    public ABC(Pointer p) {
      super(p);
    }

    public float a;
    public int b;
    public int c;

  }

  @Structure.FieldOrder({"a", "b", "c"})
  class ABCByReference extends Structure implements Structure.ByReference {
    /* Unsupported literal for constant abc */
    /* Unsupported literal for constant bac */
    /* Unsupported literal for constant cba */
    public ABCByReference() {
      super();
    }

    public ABCByReference(Pointer p) {
      super(p);
    }

    public float a;
    public int b;
    public int c;

  }



  @Structure.FieldOrder({"b", "a", "c"})
  class BAC extends Structure implements Structure.ByValue {
    /* Unsupported literal for constant abc */
    /* Unsupported literal for constant bac */
    /* Unsupported literal for constant cba */
    public BAC() {
      super();
    }

    public BAC(Pointer p) {
      super(p);
    }

    public int b;
    public float a;
    public int c;

  }

  @Structure.FieldOrder({"b", "a", "c"})
  class BACByReference extends Structure implements Structure.ByReference {
    /* Unsupported literal for constant abc */
    /* Unsupported literal for constant bac */
    /* Unsupported literal for constant cba */
    public BACByReference() {
      super();
    }

    public BACByReference(Pointer p) {
      super(p);
    }

    public int b;
    public float a;
    public int c;

  }


  void root(ABC a1, BAC a2);

}