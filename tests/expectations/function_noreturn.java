import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"f"})
  class Example extends Structure implements Structure.ByValue {
    public Example() {
      super();
    }

    public Example(Pointer p) {
      super(p);
    }

    public Callback f;

  }

  @Structure.FieldOrder({"f"})
  class ExampleByReference extends Structure implements Structure.ByReference {
    public ExampleByReference() {
      super();
    }

    public ExampleByReference(Pointer p) {
      super(p);
    }

    public Callback f;

  }


  void loop_forever();

  byte normal_return(Example arg, Callback other);

}