import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"start", "len", "point"})
  class TakeUntil_0 extends Structure implements Structure.ByValue {
    public TakeUntil_0() {
      super();
    }

    public TakeUntil_0(Pointer p) {
      super(p);
    }

    public ByteByReference start;
    public NativeLong len;
    public NativeLong point;

  }

  @Structure.FieldOrder({"start", "len", "point"})
  class TakeUntil_0ByReference extends Structure implements Structure.ByReference {
    public TakeUntil_0ByReference() {
      super();
    }

    public TakeUntil_0ByReference(Pointer p) {
      super(p);
    }

    public ByteByReference start;
    public NativeLong len;
    public NativeLong point;

  }


  TakeUntil_0 until_nul(ByteByReference start, NativeLong len);

}