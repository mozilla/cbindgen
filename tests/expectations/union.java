import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class Opaque extends PointerType {
    public Opaque() {
      super(null);
    }
    public Opaque(Pointer p) {
      super(p);
    }
  }

  class OpaqueByReference extends Opaque {
    public OpaqueByReference() {
      super(null);
    }
    public OpaqueByReference(Pointer p) {
      super(p);
    }
  }


  @Structure.FieldOrder({"x", "y"})
  class Normal extends Union implements Structure.ByValue {
    public Normal() {
      super();
    }

    public Normal(Pointer p) {
      super(p);
    }

    public int x;
    public float y;

  }

  @Structure.FieldOrder({"x", "y"})
  class NormalByReference extends Union implements Structure.ByReference {
    public NormalByReference() {
      super();
    }

    public NormalByReference(Pointer p) {
      super(p);
    }

    public int x;
    public float y;

  }



  @Structure.FieldOrder({"x", "y"})
  class NormalWithZST extends Union implements Structure.ByValue {
    public NormalWithZST() {
      super();
    }

    public NormalWithZST(Pointer p) {
      super(p);
    }

    public int x;
    public float y;

  }

  @Structure.FieldOrder({"x", "y"})
  class NormalWithZSTByReference extends Union implements Structure.ByReference {
    public NormalWithZSTByReference() {
      super();
    }

    public NormalWithZSTByReference(Pointer p) {
      super(p);
    }

    public int x;
    public float y;

  }


  void root(OpaqueByReference a, Normal b, NormalWithZST c);

}