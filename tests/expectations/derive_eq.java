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

    public _Boolean a;
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

    public _Boolean a;
    public int b;

  }



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
      Pointer p = getPointer();
      return new Bar(p.getInt(0));
    }

    public void setValue(Bar value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }


  Foo root(Bar aBar);

  class _Boolean extends IntegerType {
    public _Boolean() {
      super(1, true);
    }

    public _Boolean(long value) {
      super(1, value, true);
    }

    public _Boolean(Pointer p) {
      this(p.getByte(0));
    }

    public static final _Boolean FALSE = new _Boolean(0);
    public static final _Boolean TRUE = new _Boolean(1);
  }

  class _BooleanByReference extends ByReference {
    public _BooleanByReference() {
      super(1);
    }

    public _BooleanByReference(Pointer p) {
      super(1);
      setPointer(p);
    }

    public _Boolean getValue() {
      Pointer p = getPointer();
      return new _Boolean(p.getByte(0));
    }

    public void setValue(_Boolean value) {
      Pointer p = getPointer();
      p.setByte(0, (byte)value.intValue());
    }

  }

}