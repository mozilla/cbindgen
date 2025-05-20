import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"data", "vtable"})
  class TraitObject extends Structure implements Structure.ByValue {
    public TraitObject() {
      super();
    }

    public TraitObject(Pointer p) {
      super(p);
    }

    public Pointer data;
    public Pointer vtable;

  }

  @Structure.FieldOrder({"data", "vtable"})
  class TraitObjectByReference extends Structure implements Structure.ByReference {
    public TraitObjectByReference() {
      super();
    }

    public TraitObjectByReference(Pointer p) {
      super(p);
    }

    public Pointer data;
    public Pointer vtable;

  }


  Pointer root(Pointer ptr, TraitObject t);

}