import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class NotReprC_RefCell_i32 extends PointerType {
    public NotReprC_RefCell_i32() {
      super(null);
    }
    public NotReprC_RefCell_i32(Pointer p) {
      super(p);
    }
  }

  class NotReprC_RefCell_i32ByReference extends NotReprC_RefCell_i32 {
    public NotReprC_RefCell_i32ByReference() {
      super(null);
    }
    public NotReprC_RefCell_i32ByReference(Pointer p) {
      super(p);
    }
  }

  class Foo extends NotReprC_RefCell_i32 {
    public Foo() {
      super();
    }
    public Foo(Pointer p) {
      super(p);
    }
  }

  class FooByReference extends NotReprC_RefCell_i32ByReference {
    public FooByReference() {
      super();
    }
    public FooByReference(Pointer p) {
      super(p);
    }
  }


  @Structure.FieldOrder({"number"})
  class MyStruct extends Structure implements Structure.ByValue {
    public MyStruct() {
      super();
    }

    public MyStruct(Pointer p) {
      super(p);
    }

    public int number;

  }

  @Structure.FieldOrder({"number"})
  class MyStructByReference extends Structure implements Structure.ByReference {
    public MyStructByReference() {
      super();
    }

    public MyStructByReference(Pointer p) {
      super(p);
    }

    public int number;

  }


  void root(FooByReference a, MyStructByReference with_cell);

}