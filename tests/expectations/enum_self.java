import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"something"})
  class Foo_Bar extends Structure implements Structure.ByValue {
    public Foo_Bar() {
      super();
    }

    public Foo_Bar(Pointer p) {
      super(p);
    }

    public IntByReference something;

  }

  @Structure.FieldOrder({"something"})
  class Foo_BarByReference extends Structure implements Structure.ByReference {
    public Foo_BarByReference() {
      super();
    }

    public Foo_BarByReference(Pointer p) {
      super(p);
    }

    public IntByReference something;

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
    public static final Bar Min = new Bar(1);
    public static final Bar Max = new Bar(2);
    public static final Bar Other = new Bar(3);

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

  void root(Bar b);

}