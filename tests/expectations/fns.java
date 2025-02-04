import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"noArgs",
                         "anonymousArg",
                         "returnsNumber",
                         "namedArgs",
                         "namedArgsWildcards"})
  class Fns extends Structure implements Structure.ByValue {
    public Fns() {
      super();
    }

    public Fns(Pointer p) {
      super(p);
    }

    public Callback noArgs;
    public Callback anonymousArg;
    public Callback returnsNumber;
    public Callback namedArgs;
    public Callback namedArgsWildcards;

  }

  @Structure.FieldOrder({"noArgs",
                         "anonymousArg",
                         "returnsNumber",
                         "namedArgs",
                         "namedArgsWildcards"})
  class FnsByReference extends Structure implements Structure.ByReference {
    public FnsByReference() {
      super();
    }

    public FnsByReference(Pointer p) {
      super(p);
    }

    public Callback noArgs;
    public Callback anonymousArg;
    public Callback returnsNumber;
    public Callback namedArgs;
    public Callback namedArgsWildcards;

  }


  void root(Fns _fns);

  void no_return();

}