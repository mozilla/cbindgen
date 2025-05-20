import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;
  class Option_Foo {}



  @Structure.FieldOrder({"foo"})
  class Bar extends Structure implements Structure.ByValue {
    public Bar() {
      super();
    }

    public Bar(Pointer p) {
      super(p);
    }

    public Option_Foo foo;

  }

  @Structure.FieldOrder({"foo"})
  class BarByReference extends Structure implements Structure.ByReference {
    public BarByReference() {
      super();
    }

    public BarByReference(Pointer p) {
      super(p);
    }

    public Option_Foo foo;

  }


  void root(Bar f);

}