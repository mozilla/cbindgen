
import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"pinned_box", "pinned_ref"})
  class PinTest extends Structure implements Structure.ByValue {
    public PinTest() {
      super();
    }

    public PinTest(Pointer p) {
      super(p);
    }

    public IntByReference pinned_box;
    public IntByReference pinned_ref;

  }

  @Structure.FieldOrder({"pinned_box", "pinned_ref"})
  class PinTestByReference extends Structure implements Structure.ByReference {
    public PinTestByReference() {
      super();
    }

    public PinTestByReference(Pointer p) {
      super(p);
    }

    public IntByReference pinned_box;
    public IntByReference pinned_ref;

  }


  void root(IntByReference s, PinTest p);

}