import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"a"})
  class Foo_____u8 extends Structure implements Structure.ByValue {
    public Foo_____u8() {
      super();
    }

    public Foo_____u8(Pointer p) {
      super(p);
    }

    public ByteByReference a;

  }

  @Structure.FieldOrder({"a"})
  class Foo_____u8ByReference extends Structure implements Structure.ByReference {
    public Foo_____u8ByReference() {
      super();
    }

    public Foo_____u8ByReference(Pointer p) {
      super(p);
    }

    public ByteByReference a;

  }


  class Boo extends Foo_____u8 {
    public Boo() {
      super();
    }
    public Boo(Pointer p) {
      super(p);
    }
  }

  class BooByReference extends Foo_____u8ByReference {
    public BooByReference() {
      super();
    }
    public BooByReference(Pointer p) {
      super(p);
    }
  }

  void root(Boo x);

}