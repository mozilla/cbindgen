import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"x", "y"})
  class Dummy extends Structure implements Structure.ByValue {
    public Dummy() {
      super();
    }

    public Dummy(Pointer p) {
      super(p);
    }

    public int x;
    public float y;

  }

  @Structure.FieldOrder({"x", "y"})
  class DummyByReference extends Structure implements Structure.ByReference {
    public DummyByReference() {
      super();
    }

    public DummyByReference(Pointer p) {
      super(p);
    }

    public int x;
    public float y;

  }


  void root(Dummy d);

}