import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;



  /**
   * Constants shared by multiple CSS Box Alignment properties
   *
   * These constants match Gecko's `NS_STYLE_ALIGN_*` constants.
   */
  @Structure.FieldOrder({"bits"})
  class AlignFlags extends Structure implements Structure.ByValue {

    /**
     * 'auto'
     */
    /* Unsupported literal for constant AUTO */

    /**
     * 'normal'
     */
    /* Unsupported literal for constant NORMAL */

    /**
     * 'start'
     */
    /* Unsupported literal for constant START */

    /**
     * 'end'
     */
    /* Unsupported literal for constant END */
    /* Unsupported literal for constant ALIAS */

    /**
     * 'flex-start'
     */
    /* Unsupported literal for constant FLEX_START */
    /* Unsupported literal for constant MIXED */
    /* Unsupported literal for constant MIXED_SELF */
    public AlignFlags() {
      super();
    }

    public AlignFlags(Pointer p) {
      super(p);
    }

    public byte bits;

  }


  /**
   * Constants shared by multiple CSS Box Alignment properties
   *
   * These constants match Gecko's `NS_STYLE_ALIGN_*` constants.
   */
  @Structure.FieldOrder({"bits"})
  class AlignFlagsByReference extends Structure implements Structure.ByReference {

    /**
     * 'auto'
     */
    /* Unsupported literal for constant AUTO */

    /**
     * 'normal'
     */
    /* Unsupported literal for constant NORMAL */

    /**
     * 'start'
     */
    /* Unsupported literal for constant START */

    /**
     * 'end'
     */
    /* Unsupported literal for constant END */
    /* Unsupported literal for constant ALIAS */

    /**
     * 'flex-start'
     */
    /* Unsupported literal for constant FLEX_START */
    /* Unsupported literal for constant MIXED */
    /* Unsupported literal for constant MIXED_SELF */
    public AlignFlagsByReference() {
      super();
    }

    public AlignFlagsByReference(Pointer p) {
      super(p);
    }

    public byte bits;

  }



  @Structure.FieldOrder({"bits"})
  class DebugFlags extends Structure implements Structure.ByValue {

    /**
     * Flag with the topmost bit set of the u32
     */
    /* Unsupported literal for constant BIGGEST_ALLOWED */
    public DebugFlags() {
      super();
    }

    public DebugFlags(Pointer p) {
      super(p);
    }

    public int bits;

  }

  @Structure.FieldOrder({"bits"})
  class DebugFlagsByReference extends Structure implements Structure.ByReference {

    /**
     * Flag with the topmost bit set of the u32
     */
    /* Unsupported literal for constant BIGGEST_ALLOWED */
    public DebugFlagsByReference() {
      super();
    }

    public DebugFlagsByReference(Pointer p) {
      super(p);
    }

    public int bits;

  }



  @Structure.FieldOrder({"bits"})
  class LargeFlags extends Structure implements Structure.ByValue {

    /**
     * Flag with a very large shift that usually would be narrowed.
     */
    /* Unsupported literal for constant LARGE_SHIFT */
    /* Unsupported literal for constant INVERTED */
    public LargeFlags() {
      super();
    }

    public LargeFlags(Pointer p) {
      super(p);
    }

    public long bits;

  }

  @Structure.FieldOrder({"bits"})
  class LargeFlagsByReference extends Structure implements Structure.ByReference {

    /**
     * Flag with a very large shift that usually would be narrowed.
     */
    /* Unsupported literal for constant LARGE_SHIFT */
    /* Unsupported literal for constant INVERTED */
    public LargeFlagsByReference() {
      super();
    }

    public LargeFlagsByReference(Pointer p) {
      super(p);
    }

    public long bits;

  }



  @Structure.FieldOrder({"_0"})
  class OutOfLine extends Structure implements Structure.ByValue {
    /* Unsupported literal for constant A */
    /* Unsupported literal for constant B */
    /* Unsupported literal for constant AB */
    public OutOfLine() {
      super();
    }

    public OutOfLine(Pointer p) {
      super(p);
    }

    public int _0;

  }

  @Structure.FieldOrder({"_0"})
  class OutOfLineByReference extends Structure implements Structure.ByReference {
    /* Unsupported literal for constant A */
    /* Unsupported literal for constant B */
    /* Unsupported literal for constant AB */
    public OutOfLineByReference() {
      super();
    }

    public OutOfLineByReference(Pointer p) {
      super(p);
    }

    public int _0;

  }


  void root(AlignFlags flags, 
            DebugFlags bigger_flags, 
            LargeFlags largest_flags, 
            OutOfLine out_of_line);

}