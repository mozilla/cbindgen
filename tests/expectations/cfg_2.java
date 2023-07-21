import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;
  class Bar {}


  @Structure.FieldOrder({"w"})
  class Root extends Structure implements Structure.ByValue {
    public Root() {
      super();
    }

    public Root(Pointer p) {
      super(p);
    }

    public Bar w;

  }

  @Structure.FieldOrder({"w"})
  class RootByReference extends Structure implements Structure.ByReference {
    public RootByReference() {
      super();
    }

    public RootByReference(Pointer p) {
      super(p);
    }

    public Bar w;

  }


  void root(Root a);

}