import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("expand-dep", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"x", "y"})
  class dep_struct extends Structure implements Structure.ByValue {
    public dep_struct() {
      super();
    }

    public dep_struct(Pointer p) {
      super(p);
    }

    public int x;
    public double y;

  }

  @Structure.FieldOrder({"x", "y"})
  class dep_structByReference extends Structure implements Structure.ByReference {
    public dep_structByReference() {
      super();
    }

    public dep_structByReference(Pointer p) {
      super(p);
    }

    public int x;
    public double y;

  }


  int get_x(dep_structByReference dep_struct);

}