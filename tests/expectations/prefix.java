import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  /* Unsupported literal for constant PREFIX_LEN */


  /* Unsupported literal for constant PREFIX_X */


  /* Unsupported literal for constant PREFIX_Y */


  class PREFIX_NamedLenArray extends PointerType {
    public PREFIX_NamedLenArray() {
      super(null);
    }
    public PREFIX_NamedLenArray(Pointer p) {
      super(p);
    }
  }

  class PREFIX_NamedLenArrayByReference extends PREFIX_NamedLenArray {
    public PREFIX_NamedLenArrayByReference() {
      super(null);
    }
    public PREFIX_NamedLenArrayByReference(Pointer p) {
      super(p);
    }
  }

  class PREFIX_ValuedLenArray extends PointerType {
    public PREFIX_ValuedLenArray() {
      super(null);
    }
    public PREFIX_ValuedLenArray(Pointer p) {
      super(p);
    }
  }

  class PREFIX_ValuedLenArrayByReference extends PREFIX_ValuedLenArray {
    public PREFIX_ValuedLenArrayByReference() {
      super(null);
    }
    public PREFIX_ValuedLenArrayByReference(Pointer p) {
      super(p);
    }
  }

  class PREFIX_AbsoluteFontWeight extends IntegerType {
    public PREFIX_AbsoluteFontWeight() {
      super(4);
    }

    public PREFIX_AbsoluteFontWeight(long value) {
      super(4, value);
    }

    public PREFIX_AbsoluteFontWeight(Pointer p) {
      this(p.getInt(0));
    }
    public static final PREFIX_AbsoluteFontWeight Weight = new PREFIX_AbsoluteFontWeight(1);
    public static final PREFIX_AbsoluteFontWeight Normal = new PREFIX_AbsoluteFontWeight(2);
    public static final PREFIX_AbsoluteFontWeight Bold = new PREFIX_AbsoluteFontWeight(3);

  }

  class PREFIX_AbsoluteFontWeightByReference extends ByReference {
    public PREFIX_AbsoluteFontWeightByReference() {
      super(4);
    }

    public PREFIX_AbsoluteFontWeightByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public PREFIX_AbsoluteFontWeight getValue() {
      return new PREFIX_AbsoluteFontWeight(getPointer().getInt(0));
    }

    public void setValue(PREFIX_AbsoluteFontWeight value) {
      getPointer().setInt(0, value.intValue());
    }

  }

  void root(PREFIX_NamedLenArray x, PREFIX_ValuedLenArray y, PREFIX_AbsoluteFontWeight z);

}