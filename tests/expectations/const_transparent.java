import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class Transparent extends IntegerType {
    public Transparent() {
      super(1);
    }

    public Transparent(long value) {
      super(1, value);
    }

    public Transparent(Pointer p) {
      this(p.getByte(0));
    }

  }

  class TransparentByReference extends ByReference {
    public TransparentByReference() {
      super(1);
    }

    public TransparentByReference(Pointer p) {
      super(1);
      setPointer(p);
    }

    public Transparent getValue() {
      return new Transparent(getPointer().getByte(0));
    }

    public void setValue(Transparent value) {
      getPointer().setByte(0, (byte)value.intValue());
    }

  }

  public static final Transparent FOO  = new Transparent(0);


}