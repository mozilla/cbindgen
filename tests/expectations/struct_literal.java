import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class Bar extends PointerType {
    public Bar() {
      super(null);
    }
    public Bar(Pointer p) {
      super(p);
    }
  }

  class BarByReference extends Bar {
    public BarByReference() {
      super(null);
    }
    public BarByReference(Pointer p) {
      super(p);
    }
  }


  @Structure.FieldOrder({"a", "b"})
  class Foo extends Structure implements Structure.ByValue {
    /* Unsupported literal for constant FOO */
    /* Unsupported literal for constant FOO2 */
    /* Unsupported literal for constant FOO3 */
    public Foo() {
      super();
    }

    public Foo(Pointer p) {
      super(p);
    }

    public int a;
    public int b;

  }

  @Structure.FieldOrder({"a", "b"})
  class FooByReference extends Structure implements Structure.ByReference {
    /* Unsupported literal for constant FOO */
    /* Unsupported literal for constant FOO2 */
    /* Unsupported literal for constant FOO3 */
    public FooByReference() {
      super();
    }

    public FooByReference(Pointer p) {
      super(p);
    }

    public int a;
    public int b;

  }


  /* Unsupported literal for constant BAR */




  void root(Foo x, Bar bar);

}