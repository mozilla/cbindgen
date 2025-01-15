
import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("rename_crate", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;
  class ContainsNoExternTy {}



  @Structure.FieldOrder({"x"})
  class Foo extends Structure implements Structure.ByValue {
    public Foo() {
      super();
    }

    public Foo(Pointer p) {
      super(p);
    }

    public int x;

  }

  @Structure.FieldOrder({"x"})
  class FooByReference extends Structure implements Structure.ByReference {
    public FooByReference() {
      super();
    }

    public FooByReference(Pointer p) {
      super(p);
    }

    public int x;

  }



  @Structure.FieldOrder({"y"})
  class RenamedTy extends Structure implements Structure.ByValue {
    public RenamedTy() {
      super();
    }

    public RenamedTy(Pointer p) {
      super(p);
    }

    public long y;

  }

  @Structure.FieldOrder({"y"})
  class RenamedTyByReference extends Structure implements Structure.ByReference {
    public RenamedTyByReference() {
      super();
    }

    public RenamedTyByReference(Pointer p) {
      super(p);
    }

    public long y;

  }


  void root(Foo a);

  void renamed_func(RenamedTy a);

  void no_extern_func(ContainsNoExternTy a);

}