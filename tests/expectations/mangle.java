import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  class Bar extends IntegerType {
    public Bar() {
      super(4, true);
    }

    public Bar(long value) {
      super(4, value, true);
    }

    public Bar(Pointer p) {
      this(p.getInt(0));
    }
    public static final Bar BarSome = new Bar(1);
    public static final Bar BarThing = new Bar(2);

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
      Pointer p = getPointer();
      return new Bar(p.getInt(0));
    }

    public void setValue(Bar value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  @Structure.FieldOrder({"a"})
  class FooU8 extends Structure implements Structure.ByValue {
    public FooU8() {
      super();
    }

    public FooU8(Pointer p) {
      super(p);
    }

    public byte a;

  }

  @Structure.FieldOrder({"a"})
  class FooU8ByReference extends Structure implements Structure.ByReference {
    public FooU8ByReference() {
      super();
    }

    public FooU8ByReference(Pointer p) {
      super(p);
    }

    public byte a;

  }


  class Boo extends FooU8 {
    public Boo() {
      super();
    }
    public Boo(Pointer p) {
      super(p);
    }
  }

  class BooByReference extends FooU8ByReference {
    public BooByReference() {
      super();
    }
    public BooByReference(Pointer p) {
      super(p);
    }
  }

  void root(Boo x, Bar y);

}