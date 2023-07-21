import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class Opaque extends PointerType {
    public Opaque() {
      super(null);
    }
    public Opaque(Pointer p) {
      super(p);
    }
  }

  class OpaqueByReference extends Opaque {
    public OpaqueByReference() {
      super(null);
    }
    public OpaqueByReference(Pointer p) {
      super(p);
    }
  }

  class Option_____Opaque extends PointerType {
    public Option_____Opaque() {
      super(null);
    }
    public Option_____Opaque(Pointer p) {
      super(p);
    }
  }

  class Option_____OpaqueByReference extends Option_____Opaque {
    public Option_____OpaqueByReference() {
      super(null);
    }
    public Option_____OpaqueByReference(Pointer p) {
      super(p);
    }
  }


  @Structure.FieldOrder({"x", "y", "z", "zz"})
  class Foo extends Structure implements Structure.ByValue {
    public Foo() {
      super();
    }

    public Foo(Pointer p) {
      super(p);
    }

    public OpaqueByReference x;
    public OpaqueByReference y;
    public Callback z;
    public CallbackReference zz;

  }

  @Structure.FieldOrder({"x", "y", "z", "zz"})
  class FooByReference extends Structure implements Structure.ByReference {
    public FooByReference() {
      super();
    }

    public FooByReference(Pointer p) {
      super(p);
    }

    public OpaqueByReference x;
    public OpaqueByReference y;
    public Callback z;
    public CallbackReference zz;

  }



  @Structure.FieldOrder({"x", "y", "z", "zz"})
  class Bar extends Union implements Structure.ByValue {
    public Bar() {
      super();
    }

    public Bar(Pointer p) {
      super(p);
    }

    public OpaqueByReference x;
    public OpaqueByReference y;
    public Callback z;
    public CallbackReference zz;

  }

  @Structure.FieldOrder({"x", "y", "z", "zz"})
  class BarByReference extends Union implements Structure.ByReference {
    public BarByReference() {
      super();
    }

    public BarByReference(Pointer p) {
      super(p);
    }

    public OpaqueByReference x;
    public OpaqueByReference y;
    public Callback z;
    public CallbackReference zz;

  }


  void root(OpaqueByReference a, 
            OpaqueByReference b, 
            Foo c, 
            Bar d, 
            Option_____OpaqueByReference e, 
            Callback f);

}