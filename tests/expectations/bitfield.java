import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("bitfield", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"foo", "bar"})
  class HasBitfields extends Structure implements Structure.ByValue {
    public HasBitfields() {
      super();
    }

    public HasBitfields(Pointer p) {
      super(p);
    }

    public long foo;
    public long bar;

  }

  @Structure.FieldOrder({"foo", "bar"})
  class HasBitfieldsByReference extends Structure implements Structure.ByReference {
    public HasBitfieldsByReference() {
      super();
    }

    public HasBitfieldsByReference(Pointer p) {
      super(p);
    }

    public long foo;
    public long bar;

  }


  void root(HasBitfieldsByReference arg0);

}