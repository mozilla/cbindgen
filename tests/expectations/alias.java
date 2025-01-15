import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  class Status extends IntegerType {
    public Status() {
      super(4, true);
    }

    public Status(long value) {
      super(4, value, true);
    }

    public Status(Pointer p) {
      this(p.getInt(0));
    }
    public static final Status Ok = new Status(1);
    public static final Status Err = new Status(2);

  }

  class StatusByReference extends ByReference {
    public StatusByReference() {
      super(4);
    }

    public StatusByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public Status getValue() {
      Pointer p = getPointer();
      return new Status(p.getInt(0));
    }

    public void setValue(Status value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  @Structure.FieldOrder({"a", "b"})
  class Dep extends Structure implements Structure.ByValue {
    public Dep() {
      super();
    }

    public Dep(Pointer p) {
      super(p);
    }

    public int a;
    public float b;

  }

  @Structure.FieldOrder({"a", "b"})
  class DepByReference extends Structure implements Structure.ByReference {
    public DepByReference() {
      super();
    }

    public DepByReference(Pointer p) {
      super(p);
    }

    public int a;
    public float b;

  }



  @Structure.FieldOrder({"a", "b", "c"})
  class Foo_i32 extends Structure implements Structure.ByValue {
    public Foo_i32() {
      super();
    }

    public Foo_i32(Pointer p) {
      super(p);
    }

    public int a;
    public int b;
    public Dep c;

  }

  @Structure.FieldOrder({"a", "b", "c"})
  class Foo_i32ByReference extends Structure implements Structure.ByReference {
    public Foo_i32ByReference() {
      super();
    }

    public Foo_i32ByReference(Pointer p) {
      super(p);
    }

    public int a;
    public int b;
    public Dep c;

  }


  class IntFoo extends Foo_i32 {
    public IntFoo() {
      super();
    }
    public IntFoo(Pointer p) {
      super(p);
    }
  }

  class IntFooByReference extends Foo_i32ByReference {
    public IntFooByReference() {
      super();
    }
    public IntFooByReference(Pointer p) {
      super(p);
    }
  }


  @Structure.FieldOrder({"a", "b", "c"})
  class Foo_f64 extends Structure implements Structure.ByValue {
    public Foo_f64() {
      super();
    }

    public Foo_f64(Pointer p) {
      super(p);
    }

    public double a;
    public double b;
    public Dep c;

  }

  @Structure.FieldOrder({"a", "b", "c"})
  class Foo_f64ByReference extends Structure implements Structure.ByReference {
    public Foo_f64ByReference() {
      super();
    }

    public Foo_f64ByReference(Pointer p) {
      super(p);
    }

    public double a;
    public double b;
    public Dep c;

  }


  class DoubleFoo extends Foo_f64 {
    public DoubleFoo() {
      super();
    }
    public DoubleFoo(Pointer p) {
      super(p);
    }
  }

  class DoubleFooByReference extends Foo_f64ByReference {
    public DoubleFooByReference() {
      super();
    }
    public DoubleFooByReference(Pointer p) {
      super(p);
    }
  }


  class Unit extends IntegerType {
    public Unit() {
      super(4, false);
    }

    public Unit(long value) {
      super(4, value, false);
    }

    public Unit(Pointer p) {
      this(p.getInt(0));
    }

  }

  class UnitByReference extends ByReference {
    public UnitByReference() {
      super(4);
    }

    public UnitByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public Unit getValue() {
      Pointer p = getPointer();
      return new Unit(p.getInt(0));
    }

    public void setValue(Unit value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }


  class SpecialStatus extends Status {
    public SpecialStatus() {
      super();
    }
    public SpecialStatus(Pointer p) {
      super(p);
    }
  }

  class SpecialStatusByReference extends StatusByReference {
    public SpecialStatusByReference() {
      super();
    }
    public SpecialStatusByReference(Pointer p) {
      super(p);
    }
  }

  void root(IntFoo x, DoubleFoo y, Unit z, SpecialStatus w);

}