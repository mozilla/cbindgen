import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  /* Unsupported literal for constant CONSTANT_I64 */


  public static final float CONSTANT_FLOAT32  = 312.292f;


  public static final char DELIMITER  = ':';


  public static final char LEFTCURLY  = '{';



  @Structure.FieldOrder({"x"})
  class Foo extends Structure implements Structure.ByValue {
    /* Unsupported literal for constant CONSTANT_I64_BODY */
    public Foo() {
      super();
    }

    public Foo(Pointer p) {
      super(p);
    }

    public int x;

  }

  @Structure.FieldOrder({"x"})
  class FooByReference extends Structure implements Structure.ByReference {
    /* Unsupported literal for constant CONSTANT_I64_BODY */
    public FooByReference() {
      super();
    }

    public FooByReference(Pointer p) {
      super(p);
    }

    public int x;

  }


  /* Unsupported literal for constant SomeFoo */


}