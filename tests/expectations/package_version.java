/* Package version: 0.1.0 */
import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("package_version", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"bar"})
  class Foo extends Structure implements Structure.ByValue {
    public Foo() {
      super();
    }

    public Foo(Pointer p) {
      super(p);
    }

    public long bar;

  }

  @Structure.FieldOrder({"bar"})
  class FooByReference extends Structure implements Structure.ByReference {
    public FooByReference() {
      super();
    }

    public FooByReference(Pointer p) {
      super(p);
    }

    public long bar;

  }


  void doit(FooByReference arg0);

}