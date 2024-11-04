import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  class Foo extends IntegerType {
    public Foo() {
      super(4, true);
    }

    public Foo(long value) {
      super(4, value, true);
    }

    public Foo(Pointer p) {
      this(p.getInt(0));
    }
    public static final Foo A = new Foo(1);

  }

  class FooByReference extends ByReference {
    public FooByReference() {
      super(4);
    }

    public FooByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public Foo getValue() {
      Pointer p = getPointer();
      return new Foo(p.getInt(0));
    }

    public void setValue(Foo value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }


  void root(Foo a);

}