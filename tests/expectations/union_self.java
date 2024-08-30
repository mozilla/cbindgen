import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"something"})
  class Foo_Bar extends Structure implements Structure.ByValue {
    public Foo_Bar() {
      super();
    }

    public Foo_Bar(Pointer p) {
      super(p);
    }

    public IntByReference something;

  }

  @Structure.FieldOrder({"something"})
  class Foo_BarByReference extends Structure implements Structure.ByReference {
    public Foo_BarByReference() {
      super();
    }

    public Foo_BarByReference(Pointer p) {
      super(p);
    }

    public IntByReference something;

  }



  @Structure.FieldOrder({"something", "subexpressions"})
  class Bar extends Union implements Structure.ByValue {
    public Bar() {
      super();
    }

    public Bar(Pointer p) {
      super(p);
    }

    public int something;
    public Foo_Bar subexpressions;

  }

  @Structure.FieldOrder({"something", "subexpressions"})
  class BarByReference extends Union implements Structure.ByReference {
    public BarByReference() {
      super();
    }

    public BarByReference(Pointer p) {
      super(p);
    }

    public int something;
    public Foo_Bar subexpressions;

  }


  void root(Bar b);

}