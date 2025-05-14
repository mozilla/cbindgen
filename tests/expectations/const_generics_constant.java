import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  /* Unsupported literal for constant FONT_WEIGHT_FRACTION_BITS */



  @Structure.FieldOrder({"value"})
  class FixedPoint_FONT_WEIGHT_FRACTION_BITS extends Structure implements Structure.ByValue {
    public FixedPoint_FONT_WEIGHT_FRACTION_BITS() {
      super();
    }

    public FixedPoint_FONT_WEIGHT_FRACTION_BITS(Pointer p) {
      super(p);
    }

    public short value;

  }

  @Structure.FieldOrder({"value"})
  class FixedPoint_FONT_WEIGHT_FRACTION_BITSByReference extends Structure implements Structure.ByReference {
    public FixedPoint_FONT_WEIGHT_FRACTION_BITSByReference() {
      super();
    }

    public FixedPoint_FONT_WEIGHT_FRACTION_BITSByReference(Pointer p) {
      super(p);
    }

    public short value;

  }


  class FontWeightFixedPoint extends FixedPoint_FONT_WEIGHT_FRACTION_BITS {
    public FontWeightFixedPoint() {
      super();
    }
    public FontWeightFixedPoint(Pointer p) {
      super(p);
    }
  }

  class FontWeightFixedPointByReference extends FixedPoint_FONT_WEIGHT_FRACTION_BITSByReference {
    public FontWeightFixedPointByReference() {
      super();
    }
    public FontWeightFixedPointByReference(Pointer p) {
      super(p);
    }
  }


  @Structure.FieldOrder({"_0"})
  class FontWeight extends Structure implements Structure.ByValue {
    /* Unsupported literal for constant NORMAL */
    public FontWeight() {
      super();
    }

    public FontWeight(Pointer p) {
      super(p);
    }

    public FontWeightFixedPoint _0;

  }

  @Structure.FieldOrder({"_0"})
  class FontWeightByReference extends Structure implements Structure.ByReference {
    /* Unsupported literal for constant NORMAL */
    public FontWeightByReference() {
      super();
    }

    public FontWeightByReference(Pointer p) {
      super(p);
    }

    public FontWeightFixedPoint _0;

  }


  void root(FontWeight w);

}