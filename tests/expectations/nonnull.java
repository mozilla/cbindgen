import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class Opaque extends PointerType {
    public Opaque() {
      super(null);
    }
    public Opaque(Pointer p) {
      super(p);
    }
  }

  class OpaqueByReference extends Opaque {
    public OpaqueByReference() {
      super(null);
    }
    public OpaqueByReference(Pointer p) {
      super(p);
    }
  }


  @Structure.FieldOrder({"a", "b", "c", "d", "e", "f", "g", "h", "i"})
  class Foo_u64 extends Structure implements Structure.ByValue {
    public Foo_u64() {
      super();
    }

    public Foo_u64(Pointer p) {
      super(p);
    }

    public FloatByReference a;
    public LongByReference b;
    public OpaqueByReference c;
    public PointerByReference d;
    public PointerByReference e;
    public PointerByReference f;
    public LongByReference g;
    public IntByReference h;
    public PointerByReference i;

  }

  @Structure.FieldOrder({"a", "b", "c", "d", "e", "f", "g", "h", "i"})
  class Foo_u64ByReference extends Structure implements Structure.ByReference {
    public Foo_u64ByReference() {
      super();
    }

    public Foo_u64ByReference(Pointer p) {
      super(p);
    }

    public FloatByReference a;
    public LongByReference b;
    public OpaqueByReference c;
    public PointerByReference d;
    public PointerByReference e;
    public PointerByReference f;
    public LongByReference g;
    public IntByReference h;
    public PointerByReference i;

  }


  void root(IntByReference arg, Foo_u64ByReference foo, PointerByReference d);

}