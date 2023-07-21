import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


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


  int foo();

  void bar(Normal a);

  int baz();

}