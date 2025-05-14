import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"x", "y"})
  class Foo_i32__i32 extends Structure implements Structure.ByValue {
    public Foo_i32__i32() {
      super();
    }

    public Foo_i32__i32(Pointer p) {
      super(p);
    }

    public int x;
    public int y;

  }

  @Structure.FieldOrder({"x", "y"})
  class Foo_i32__i32ByReference extends Structure implements Structure.ByReference {
    public Foo_i32__i32ByReference() {
      super();
    }

    public Foo_i32__i32ByReference(Pointer p) {
      super(p);
    }

    public int x;
    public int y;

  }


  class IntFoo_i32 extends Foo_i32__i32 {
    public IntFoo_i32() {
      super();
    }
    public IntFoo_i32(Pointer p) {
      super(p);
    }
  }

  class IntFoo_i32ByReference extends Foo_i32__i32ByReference {
    public IntFoo_i32ByReference() {
      super();
    }
    public IntFoo_i32ByReference(Pointer p) {
      super(p);
    }
  }

  void root(IntFoo_i32 a);

}