import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  class Foo extends Structure implements Structure.ByValue {
    /* Unsupported literal for constant GA */
    public static final float ZO  = 3.14f;
    public Foo() {
      super();
    }

    public Foo(Pointer p) {
      super(p);
    }


  }

  class FooByReference extends Structure implements Structure.ByReference {
    /* Unsupported literal for constant GA */
    public static final float ZO  = 3.14f;
    public FooByReference() {
      super();
    }

    public FooByReference(Pointer p) {
      super(p);
    }


  }


  void root(Foo x);

}