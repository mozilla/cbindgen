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
  class StyleAlignFlags extends Structure implements Structure.ByValue {

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
    public StyleAlignFlags() {
      super();
    }

    public StyleAlignFlags(Pointer p) {
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
  class StyleAlignFlagsByReference extends Structure implements Structure.ByReference {

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
    public StyleAlignFlagsByReference() {
      super();
    }

    public StyleAlignFlagsByReference(Pointer p) {
      super(p);
    }

    public byte bits;

  }




  /**
   * An arbitrary identifier for a native (OS compositor) surface
   */
  @Structure.FieldOrder({"_0"})
  class StyleNativeSurfaceId extends Structure implements Structure.ByValue {

    /**
     * A special id for the native surface that is used for debug / profiler overlays.
     */
    /* Unsupported literal for constant DEBUG_OVERLAY */
    public StyleNativeSurfaceId() {
      super();
    }

    public StyleNativeSurfaceId(Pointer p) {
      super(p);
    }

    public long _0;

  }


  /**
   * An arbitrary identifier for a native (OS compositor) surface
   */
  @Structure.FieldOrder({"_0"})
  class StyleNativeSurfaceIdByReference extends Structure implements Structure.ByReference {

    /**
     * A special id for the native surface that is used for debug / profiler overlays.
     */
    /* Unsupported literal for constant DEBUG_OVERLAY */
    public StyleNativeSurfaceIdByReference() {
      super();
    }

    public StyleNativeSurfaceIdByReference(Pointer p) {
      super(p);
    }

    public long _0;

  }



  @Structure.FieldOrder({"surface_id", "x", "y"})
  class StyleNativeTileId extends Structure implements Structure.ByValue {

    /**
     * A special id for the native surface that is used for debug / profiler overlays.
     */
    /* Unsupported literal for constant DEBUG_OVERLAY */
    public StyleNativeTileId() {
      super();
    }

    public StyleNativeTileId(Pointer p) {
      super(p);
    }

    public StyleNativeSurfaceId surface_id;
    public int x;
    public int y;

  }

  @Structure.FieldOrder({"surface_id", "x", "y"})
  class StyleNativeTileIdByReference extends Structure implements Structure.ByReference {

    /**
     * A special id for the native surface that is used for debug / profiler overlays.
     */
    /* Unsupported literal for constant DEBUG_OVERLAY */
    public StyleNativeTileIdByReference() {
      super();
    }

    public StyleNativeTileIdByReference(Pointer p) {
      super(p);
    }

    public StyleNativeSurfaceId surface_id;
    public int x;
    public int y;

  }


  void root(StyleAlignFlags flags, StyleNativeTileId tile);

}