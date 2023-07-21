import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"a", "b"})
  class PREFIXFoo extends Structure implements Structure.ByValue {
    /* Unsupported literal for constant FOO */
    public PREFIXFoo() {
      super();
    }

    public PREFIXFoo(Pointer p) {
      super(p);
    }

    public int a;
    public int b;

  }

  @Structure.FieldOrder({"a", "b"})
  class PREFIXFooByReference extends Structure implements Structure.ByReference {
    /* Unsupported literal for constant FOO */
    public PREFIXFooByReference() {
      super();
    }

    public PREFIXFooByReference(Pointer p) {
      super(p);
    }

    public int a;
    public int b;

  }


  /* Unsupported literal for constant PREFIXBAR */


  void root(PREFIXFoo x);

}