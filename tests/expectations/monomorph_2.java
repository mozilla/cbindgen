import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class A extends PointerType {
    public A() {
      super(null);
    }
    public A(Pointer p) {
      super(p);
    }
  }

  class AByReference extends A {
    public AByReference() {
      super(null);
    }
    public AByReference(Pointer p) {
      super(p);
    }
  }

  class B extends PointerType {
    public B() {
      super(null);
    }
    public B(Pointer p) {
      super(p);
    }
  }

  class BByReference extends B {
    public BByReference() {
      super(null);
    }
    public BByReference(Pointer p) {
      super(p);
    }
  }


  @Structure.FieldOrder({"members", "count"})
  class List_A extends Structure implements Structure.ByValue {
    public List_A() {
      super();
    }

    public List_A(Pointer p) {
      super(p);
    }

    public AByReference members;
    public _Size count;

  }

  @Structure.FieldOrder({"members", "count"})
  class List_AByReference extends Structure implements Structure.ByReference {
    public List_AByReference() {
      super();
    }

    public List_AByReference(Pointer p) {
      super(p);
    }

    public AByReference members;
    public _Size count;

  }



  @Structure.FieldOrder({"members", "count"})
  class List_B extends Structure implements Structure.ByValue {
    public List_B() {
      super();
    }

    public List_B(Pointer p) {
      super(p);
    }

    public BByReference members;
    public _Size count;

  }

  @Structure.FieldOrder({"members", "count"})
  class List_BByReference extends Structure implements Structure.ByReference {
    public List_BByReference() {
      super();
    }

    public List_BByReference(Pointer p) {
      super(p);
    }

    public BByReference members;
    public _Size count;

  }


  void foo(List_A a);

  void bar(List_B b);

  class _Size extends IntegerType {
    public _Size() {
      super(Native.POINTER_SIZE, true);
    }

    public _Size(long value) {
      super(Native.POINTER_SIZE, value, true);
    }

    public _Size(Pointer p) {
      this(Native.POINTER_SIZE == 8 ? p.getLong(0) : p.getInt(0));
    }

  }

  class _SizeByReference extends ByReference {
    public _SizeByReference() {
      super(Native.POINTER_SIZE);
    }

    public _SizeByReference(Pointer p) {
      super(Native.POINTER_SIZE);
      setPointer(p);
    }

    public _Size getValue() {
      Pointer p = getPointer();
      return new _Size(Native.POINTER_SIZE == 8 ? p.getLong(0) : p.getInt(0));
    }

    public void setValue(_Size value) {
      Pointer p = getPointer();
      if (Native.POINTER_SIZE == 8) { p.setLong(0, value.longValue()); } else { p.setInt(0, value.intValue()); }
    }

  }

}