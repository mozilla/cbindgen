import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("workspace", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  /* Unsupported literal for constant EXT_CONST */



  @Structure.FieldOrder({"data"})
  class ExtType extends Structure implements Structure.ByValue {
    public ExtType() {
      super();
    }

    public ExtType(Pointer p) {
      super(p);
    }

    public int data;

  }

  @Structure.FieldOrder({"data"})
  class ExtTypeByReference extends Structure implements Structure.ByReference {
    public ExtTypeByReference() {
      super();
    }

    public ExtTypeByReference(Pointer p) {
      super(p);
    }

    public int data;

  }


  void consume_ext(ExtType _ext);

}