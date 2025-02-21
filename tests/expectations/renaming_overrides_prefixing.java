import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class StyleA extends PointerType {
    public StyleA() {
      super(null);
    }
    public StyleA(Pointer p) {
      super(p);
    }
  }

  class StyleAByReference extends StyleA {
    public StyleAByReference() {
      super(null);
    }
    public StyleAByReference(Pointer p) {
      super(p);
    }
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


  void root(StyleAByReference a, B b);

}