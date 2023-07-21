import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"a"})
  class Foo extends Structure implements Structure.ByValue {
    public Foo() {
      super();
    }

    public Foo(Pointer p) {
      super(p);
    }

    public char a;

  }

  @Structure.FieldOrder({"a"})
  class FooByReference extends Structure implements Structure.ByReference {
    public FooByReference() {
      super();
    }

    public FooByReference(Pointer p) {
      super(p);
    }

    public char a;

  }


  void root(Foo a);

}