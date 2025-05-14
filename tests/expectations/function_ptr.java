import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  interface MyCallback extends Callback {
    void invoke(_Size a, _Size b);
  }

  interface MyOtherCallback extends Callback {
    void invoke(_Size a, _Size lot, _Size of, _Size args, _Size and_then_some);
  }

  void my_function(MyCallback a, MyOtherCallback b);

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