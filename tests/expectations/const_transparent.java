import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  class TransparentStruct extends IntegerType {
    public TransparentStruct() {
      super(1, true);
    }

    public TransparentStruct(long value) {
      super(1, value, true);
    }

    public TransparentStruct(Pointer p) {
      this(p.getByte(0));
    }

  }

  class TransparentStructByReference extends ByReference {
    public TransparentStructByReference() {
      super(1);
    }

    public TransparentStructByReference(Pointer p) {
      super(1);
      setPointer(p);
    }

    public TransparentStruct getValue() {
      Pointer p = getPointer();
      return new TransparentStruct(p.getByte(0));
    }

    public void setValue(TransparentStruct value) {
      Pointer p = getPointer();
      p.setByte(0, (byte)value.intValue());
    }

  }

  /* Unsupported literal for constant ASSOC_STRUCT_FOO */

  public static final TransparentStruct ASSOC_STRUCT_BAR  = new TransparentStruct(2);




  class TransparentTupleStruct extends IntegerType {
    public TransparentTupleStruct() {
      super(1, true);
    }

    public TransparentTupleStruct(long value) {
      super(1, value, true);
    }

    public TransparentTupleStruct(Pointer p) {
      this(p.getByte(0));
    }

  }

  class TransparentTupleStructByReference extends ByReference {
    public TransparentTupleStructByReference() {
      super(1);
    }

    public TransparentTupleStructByReference(Pointer p) {
      super(1);
      setPointer(p);
    }

    public TransparentTupleStruct getValue() {
      Pointer p = getPointer();
      return new TransparentTupleStruct(p.getByte(0));
    }

    public void setValue(TransparentTupleStruct value) {
      Pointer p = getPointer();
      p.setByte(0, (byte)value.intValue());
    }

  }


  public static final TransparentStruct STRUCT_FOO  = new TransparentStruct(4);


  public static final TransparentTupleStruct STRUCT_BAR  = new TransparentTupleStruct(5);






}