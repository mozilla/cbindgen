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
  class Normal extends Structure implements Structure.ByValue {
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
  class NormalByReference extends Structure implements Structure.ByReference {
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
  class NormalWithZST extends Structure implements Structure.ByValue {
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
  class NormalWithZSTByReference extends Structure implements Structure.ByReference {
    public NormalWithZSTByReference() {
      super();
    }

    public NormalWithZSTByReference(Pointer p) {
      super(p);
    }

    public int x;
    public float y;

  }



  @Structure.FieldOrder({"m0", "m1"})
  class TupleRenamed extends Structure implements Structure.ByValue {
    public TupleRenamed() {
      super();
    }

    public TupleRenamed(Pointer p) {
      super(p);
    }

    public int m0;
    public float m1;

  }

  @Structure.FieldOrder({"m0", "m1"})
  class TupleRenamedByReference extends Structure implements Structure.ByReference {
    public TupleRenamedByReference() {
      super();
    }

    public TupleRenamedByReference(Pointer p) {
      super(p);
    }

    public int m0;
    public float m1;

  }



  @Structure.FieldOrder({"x", "y"})
  class TupleNamed extends Structure implements Structure.ByValue {
    public TupleNamed() {
      super();
    }

    public TupleNamed(Pointer p) {
      super(p);
    }

    public int x;
    public float y;

  }

  @Structure.FieldOrder({"x", "y"})
  class TupleNamedByReference extends Structure implements Structure.ByReference {
    public TupleNamedByReference() {
      super();
    }

    public TupleNamedByReference(Pointer p) {
      super(p);
    }

    public int x;
    public float y;

  }


  void root(OpaqueByReference a, Normal b, NormalWithZST c, TupleRenamed d, TupleNamed e);

}