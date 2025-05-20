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
      super(4, true);
    }

    public IE(long value) {
      super(4, value, true);
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
      Pointer p = getPointer();
      return new IE(p.getInt(0));
    }

    public void setValue(IE value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class UE extends IntegerType {
    public UE() {
      super(4, true);
    }

    public UE(long value) {
      super(4, value, true);
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
      Pointer p = getPointer();
      return new UE(p.getInt(0));
    }

    public void setValue(UE value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class Usize extends IntegerType {
    public Usize() {
      super(Native.POINTER_SIZE, true);
    }

    public Usize(long value) {
      super(Native.POINTER_SIZE, value, true);
    }

    public Usize(Pointer p) {
      this(Native.POINTER_SIZE == 8 ? p.getLong(0) : p.getInt(0));
    }

  }

  class UsizeByReference extends ByReference {
    public UsizeByReference() {
      super(Native.POINTER_SIZE);
    }

    public UsizeByReference(Pointer p) {
      super(Native.POINTER_SIZE);
      setPointer(p);
    }

    public Usize getValue() {
      Pointer p = getPointer();
      return new Usize(Native.POINTER_SIZE == 8 ? p.getLong(0) : p.getInt(0));
    }

    public void setValue(Usize value) {
      Pointer p = getPointer();
      if (Native.POINTER_SIZE == 8) { p.setLong(0, value.longValue()); } else { p.setInt(0, value.intValue()); }
    }

  }



  class Isize extends IntegerType {
    public Isize() {
      super(Native.POINTER_SIZE, false);
    }

    public Isize(long value) {
      super(Native.POINTER_SIZE, value, false);
    }

    public Isize(Pointer p) {
      this(Native.POINTER_SIZE == 8 ? p.getLong(0) : p.getInt(0));
    }

  }

  class IsizeByReference extends ByReference {
    public IsizeByReference() {
      super(Native.POINTER_SIZE);
    }

    public IsizeByReference(Pointer p) {
      super(Native.POINTER_SIZE);
      setPointer(p);
    }

    public Isize getValue() {
      Pointer p = getPointer();
      return new Isize(Native.POINTER_SIZE == 8 ? p.getLong(0) : p.getInt(0));
    }

    public void setValue(Isize value) {
      Pointer p = getPointer();
      if (Native.POINTER_SIZE == 8) { p.setLong(0, value.longValue()); } else { p.setInt(0, value.intValue()); }
    }

  }


  void root(Usize arg0, Isize arg1, UE arg2, IE arg3);

}