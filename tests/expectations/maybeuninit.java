
import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class NotReprC______i32 extends PointerType {
    public NotReprC______i32() {
      super(null);
    }
    public NotReprC______i32(Pointer p) {
      super(p);
    }
  }

  class NotReprC______i32ByReference extends NotReprC______i32 {
    public NotReprC______i32ByReference() {
      super(null);
    }
    public NotReprC______i32ByReference(Pointer p) {
      super(p);
    }
  }

  class Foo extends NotReprC______i32 {
    public Foo() {
      super();
    }
    public Foo(Pointer p) {
      super(p);
    }
  }

  class FooByReference extends NotReprC______i32ByReference {
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

    public IntByReference number;

  }

  @Structure.FieldOrder({"number"})
  class MyStructByReference extends Structure implements Structure.ByReference {
    public MyStructByReference() {
      super();
    }

    public MyStructByReference(Pointer p) {
      super(p);
    }

    public IntByReference number;

  }


  void root(FooByReference a, MyStructByReference with_maybe_uninit);

}