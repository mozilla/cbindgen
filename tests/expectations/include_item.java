import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"x", "y"})
  class A extends Structure implements Structure.ByValue {
    public A() {
      super();
    }

    public A(Pointer p) {
      super(p);
    }

    public int x;
    public float y;

  }

  @Structure.FieldOrder({"x", "y"})
  class AByReference extends Structure implements Structure.ByReference {
    public AByReference() {
      super();
    }

    public AByReference(Pointer p) {
      super(p);
    }

    public int x;
    public float y;

  }



  @Structure.FieldOrder({"data"})
  class B extends Structure implements Structure.ByValue {
    public B() {
      super();
    }

    public B(Pointer p) {
      super(p);
    }

    public A data;

  }

  @Structure.FieldOrder({"data"})
  class BByReference extends Structure implements Structure.ByReference {
    public BByReference() {
      super();
    }

    public BByReference(Pointer p) {
      super(p);
    }

    public A data;

  }


}