
import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class MaybeOwnedPtr_i32 extends IntegerType {
    public MaybeOwnedPtr_i32() {
      super(4);
    }

    public MaybeOwnedPtr_i32(long value) {
      super(4, value);
    }

    public MaybeOwnedPtr_i32(Pointer p) {
      this(p.getInt(0));
    }
    public static final MaybeOwnedPtr_i32 Owned_i32 = new MaybeOwnedPtr_i32(1);
    public static final MaybeOwnedPtr_i32 None_i32 = new MaybeOwnedPtr_i32(2);

  }

  class MaybeOwnedPtr_i32ByReference extends ByReference {
    public MaybeOwnedPtr_i32ByReference() {
      super(4);
    }

    public MaybeOwnedPtr_i32ByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public MaybeOwnedPtr_i32 getValue() {
      return new MaybeOwnedPtr_i32(getPointer().getInt(0));
    }

    public void setValue(MaybeOwnedPtr_i32 value) {
      getPointer().setInt(0, value.intValue());
    }

  }


  @Structure.FieldOrder({"ptr"})
  class OwnedPtr_i32 extends Structure implements Structure.ByValue {
    public OwnedPtr_i32() {
      super();
    }

    public OwnedPtr_i32(Pointer p) {
      super(p);
    }

    public IntByReference ptr;

  }

  @Structure.FieldOrder({"ptr"})
  class OwnedPtr_i32ByReference extends Structure implements Structure.ByReference {
    public OwnedPtr_i32ByReference() {
      super();
    }

    public OwnedPtr_i32ByReference(Pointer p) {
      super(p);
    }

    public IntByReference ptr;

  }


  MaybeOwnedPtr_i32 maybe_consume(OwnedPtr_i32 input);

}