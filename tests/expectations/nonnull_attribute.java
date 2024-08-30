
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


  @Structure.FieldOrder({"a", "b", "c", "d"})
  class References extends Structure implements Structure.ByValue {
    public References() {
      super();
    }

    public References(Pointer p) {
      super(p);
    }

    public OpaqueByReference a;
    public OpaqueByReference b;
    public OpaqueByReference c;
    public OpaqueByReference d;

  }

  @Structure.FieldOrder({"a", "b", "c", "d"})
  class ReferencesByReference extends Structure implements Structure.ByReference {
    public ReferencesByReference() {
      super();
    }

    public ReferencesByReference(Pointer p) {
      super(p);
    }

    public OpaqueByReference a;
    public OpaqueByReference b;
    public OpaqueByReference c;
    public OpaqueByReference d;

  }



  @Structure.FieldOrder({"a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k"})
  class Pointers_u64 extends Structure implements Structure.ByValue {
    public Pointers_u64() {
      super();
    }

    public Pointers_u64(Pointer p) {
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
    public LongByReference j;
    public LongByReference k;

  }

  @Structure.FieldOrder({"a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k"})
  class Pointers_u64ByReference extends Structure implements Structure.ByReference {
    public Pointers_u64ByReference() {
      super();
    }

    public Pointers_u64ByReference(Pointer p) {
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
    public LongByReference j;
    public LongByReference k;

  }


  void value_arg(References arg);

  void mutltiple_args(IntByReference arg, Pointers_u64ByReference foo, PointerByReference d);

  void ref_arg(Pointers_u64ByReference arg);

  void mut_ref_arg(Pointers_u64ByReference arg);

  void optional_ref_arg(Pointers_u64ByReference arg);

  void optional_mut_ref_arg(Pointers_u64ByReference arg);

  void nullable_const_ptr(Pointers_u64ByReference arg);

  void nullable_mut_ptr(Pointers_u64ByReference arg);

}