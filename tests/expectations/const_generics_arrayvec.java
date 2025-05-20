import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"xs", "len"})
  class ArrayVec_____u8__100 extends Structure implements Structure.ByValue {
    public ArrayVec_____u8__100() {
      super();
    }

    public ArrayVec_____u8__100(Pointer p) {
      super(p);
    }

    public ByteByReference[] xs;
    public int len;

  }

  @Structure.FieldOrder({"xs", "len"})
  class ArrayVec_____u8__100ByReference extends Structure implements Structure.ByReference {
    public ArrayVec_____u8__100ByReference() {
      super();
    }

    public ArrayVec_____u8__100ByReference(Pointer p) {
      super(p);
    }

    public ByteByReference[] xs;
    public int len;

  }


  int push(ArrayVec_____u8__100ByReference v, ByteByReference elem);

}