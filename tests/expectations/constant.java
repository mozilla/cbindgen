import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  /* Unsupported literal for constant FOO */


  public static final char DELIMITER  = ':';


  public static final char LEFTCURLY  = '{';


  public static final char QUOTE  = '\'';


  public static final char TAB  = '\t';


  public static final char NEWLINE  = '\n';


  /* Unsupported literal for constant HEART */


  /* Unsupported literal for constant EQUID */


  public static final float ZOM  = 3.14f;



  /**
   * A single-line doc comment.
   */
  /* Unsupported literal for constant POS_ONE */



  /**
   * A
   * multi-line
   * doc
   * comment.
   */
  /* Unsupported literal for constant NEG_ONE */


  /* Unsupported literal for constant SHIFT */


  /* Unsupported literal for constant XBOOL */


  /* Unsupported literal for constant XFALSE */


  /* Unsupported literal for constant XTRUE */


  /* Unsupported literal for constant CAST */


  /* Unsupported literal for constant DOUBLE_CAST */



  @Structure.FieldOrder({"x"})
  class Foo extends Structure implements Structure.ByValue {
    public Foo() {
      super();
    }

    public Foo(Pointer p) {
      super(p);
    }

    public int[] x;

  }

  @Structure.FieldOrder({"x"})
  class FooByReference extends Structure implements Structure.ByReference {
    public FooByReference() {
      super();
    }

    public FooByReference(Pointer p) {
      super(p);
    }

    public int[] x;

  }


  void root(Foo x);

}