
import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class Option_i64 extends PointerType {
    public Option_i64() {
      super(null);
    }
    public Option_i64(Pointer p) {
      super(p);
    }
  }

  class Option_i64ByReference extends Option_i64 {
    public Option_i64ByReference() {
      super(null);
    }
    public Option_i64ByReference(Pointer p) {
      super(p);
    }
  }


  @Structure.FieldOrder({"a", "b", "c", "d", "e", "f", "g", "h", "i", "j"})
  class NonZeroTest extends Structure implements Structure.ByValue {
    public NonZeroTest() {
      super();
    }

    public NonZeroTest(Pointer p) {
      super(p);
    }

    public byte a;
    public short b;
    public int c;
    public long d;
    public byte e;
    public short f;
    public int g;
    public long h;
    public long i;
    public Option_i64ByReference j;

  }

  @Structure.FieldOrder({"a", "b", "c", "d", "e", "f", "g", "h", "i", "j"})
  class NonZeroTestByReference extends Structure implements Structure.ByReference {
    public NonZeroTestByReference() {
      super();
    }

    public NonZeroTestByReference(Pointer p) {
      super(p);
    }

    public byte a;
    public short b;
    public int c;
    public long d;
    public byte e;
    public short f;
    public int g;
    public long h;
    public long i;
    public Option_i64ByReference j;

  }


  void root(NonZeroTest test, 
            byte a, 
            short b, 
            int c, 
            long d, 
            byte e, 
            short f, 
            int g, 
            long h, 
            long i, 
            Option_i64ByReference j);

}