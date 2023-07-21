import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("derive-eq", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"a", "b"})
  class Foo extends Structure implements Structure.ByValue {
    public Foo() {
      super();
    }

    public Foo(Pointer p) {
      super(p);
    }

    public boolean a;
    public int b;

  }

  @Structure.FieldOrder({"a", "b"})
  class FooByReference extends Structure implements Structure.ByReference {
    public FooByReference() {
      super();
    }

    public FooByReference(Pointer p) {
      super(p);
    }

    public boolean a;
    public int b;

  }


  class Bar extends IntegerType {
    public Bar() {
      super(4);
    }

    public Bar(long value) {
      super(4, value);
    }

    public Bar(Pointer p) {
      this(p.getInt(0));
    }
    public static final Bar Baz = new Bar(1);
    public static final Bar Bazz = new Bar(2);
    public static final Bar FooNamed = new Bar(3);
    public static final Bar FooParen = new Bar(4);

  }

  class BarByReference extends ByReference {
    public BarByReference() {
      super(4);
    }

    public BarByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public Bar getValue() {
      return new Bar(getPointer().getInt(0));
    }

    public void setValue(Bar value) {
      getPointer().setInt(0, value.intValue());
    }

  }

  Foo root(Bar aBar);

}