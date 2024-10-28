import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("mod_path", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  /* Unsupported literal for constant EXPORT_ME_TOO */



  @Structure.FieldOrder({"val"})
  class ExportMe extends Structure implements Structure.ByValue {
    public ExportMe() {
      super();
    }

    public ExportMe(Pointer p) {
      super(p);
    }

    public long val;

  }

  @Structure.FieldOrder({"val"})
  class ExportMeByReference extends Structure implements Structure.ByReference {
    public ExportMeByReference() {
      super();
    }

    public ExportMeByReference(Pointer p) {
      super(p);
    }

    public long val;

  }


  void export_me(ExportMeByReference val);

}