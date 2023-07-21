import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"a"})
  class PREFIXBar extends Structure implements Structure.ByValue {
    public PREFIXBar() {
      super();
    }

    public PREFIXBar(Pointer p) {
      super(p);
    }

    public int a;

  }

  @Structure.FieldOrder({"a"})
  class PREFIXBarByReference extends Structure implements Structure.ByReference {
    public PREFIXBarByReference() {
      super();
    }

    public PREFIXBarByReference(Pointer p) {
      super(p);
    }

    public int a;

  }



  @Structure.FieldOrder({"a", "b", "bar"})
  class PREFIXFoo extends Structure implements Structure.ByValue {
    public PREFIXFoo() {
      super();
    }

    public PREFIXFoo(Pointer p) {
      super(p);
    }

    public int a;
    public int b;
    public PREFIXBar bar;

  }

  @Structure.FieldOrder({"a", "b", "bar"})
  class PREFIXFooByReference extends Structure implements Structure.ByReference {
    public PREFIXFooByReference() {
      super();
    }

    public PREFIXFooByReference(Pointer p) {
      super(p);
    }

    public int a;
    public int b;
    public PREFIXBar bar;

  }


  /* Unsupported literal for constant PREFIXVAL */


  void root(PREFIXFoo x);

}