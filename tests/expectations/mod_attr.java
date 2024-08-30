
import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("mod_attr", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  /* begin condition not supported  Some(Define("FOO")) */
  /* Unsupported literal for constant FOO */

  /* end condition not supported  Some(Define("FOO")) */

  /* begin condition not supported  Some(Define("BAR")) */
  /* Unsupported literal for constant BAR */

  /* end condition not supported  Some(Define("BAR")) */


  class Foo extends Structure implements Structure.ByValue {
    public Foo() {
      super();
    }

    public Foo(Pointer p) {
      super(p);
    }


  }

  class FooByReference extends Structure implements Structure.ByReference {
    public FooByReference() {
      super();
    }

    public FooByReference(Pointer p) {
      super(p);
    }


  }



  class Bar extends Structure implements Structure.ByValue {
    public Bar() {
      super();
    }

    public Bar(Pointer p) {
      super(p);
    }


  }

  class BarByReference extends Structure implements Structure.ByReference {
    public BarByReference() {
      super();
    }

    public BarByReference(Pointer p) {
      super(p);
    }


  }


  void foo(FooByReference foo);

  void bar(BarByReference bar);

}