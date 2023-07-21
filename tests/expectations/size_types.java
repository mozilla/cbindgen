import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class IE extends IntegerType {
    public IE() {
      super(4);
    }

    public IE(long value) {
      super(4, value);
    }

    public IE(Pointer p) {
      this(p.getInt(0));
    }
    public static final IE IV = new IE(1);

  }

  class IEByReference extends ByReference {
    public IEByReference() {
      super(4);
    }

    public IEByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public IE getValue() {
      return new IE(getPointer().getInt(0));
    }

    public void setValue(IE value) {
      getPointer().setInt(0, value.intValue());
    }

  }

  class UE extends IntegerType {
    public UE() {
      super(4);
    }

    public UE(long value) {
      super(4, value);
    }

    public UE(Pointer p) {
      this(p.getInt(0));
    }
    public static final UE UV = new UE(1);

  }

  class UEByReference extends ByReference {
    public UEByReference() {
      super(4);
    }

    public UEByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public UE getValue() {
      return new UE(getPointer().getInt(0));
    }

    public void setValue(UE value) {
      getPointer().setInt(0, value.intValue());
    }

  }

  class Usize extends IntegerType {
    public Usize() {
      super(Native.SIZE_T_SIZE);
    }

    public Usize(long value) {
      super(Native.SIZE_T_SIZE, value);
    }

    public Usize(Pointer p) {
      this(p.getNativeLong(0).longValue());
    }

  }

  class UsizeByReference extends ByReference {
    public UsizeByReference() {
      super(Native.SIZE_T_SIZE);
    }

    public UsizeByReference(Pointer p) {
      super(Native.SIZE_T_SIZE);
      setPointer(p);
    }

    public Usize getValue() {
      return new Usize(getPointer().getNativeLong(0).longValue());
    }

    public void setValue(Usize value) {
      getPointer().setNativeLong(0, new NativeLong(value.longValue()));
    }

  }

  class Isize extends IntegerType {
    public Isize() {
      super(Native.SIZE_T_SIZE);
    }

    public Isize(long value) {
      super(Native.SIZE_T_SIZE, value);
    }

    public Isize(Pointer p) {
      this(p.getNativeLong(0).longValue());
    }

  }

  class IsizeByReference extends ByReference {
    public IsizeByReference() {
      super(Native.SIZE_T_SIZE);
    }

    public IsizeByReference(Pointer p) {
      super(Native.SIZE_T_SIZE);
      setPointer(p);
    }

    public Isize getValue() {
      return new Isize(getPointer().getNativeLong(0).longValue());
    }

    public void setValue(Isize value) {
      getPointer().setNativeLong(0, new NativeLong(value.longValue()));
    }

  }

  void root(Usize arg0, Isize arg1, UE arg2, IE arg3);

}