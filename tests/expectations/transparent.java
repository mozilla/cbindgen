import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class DummyStruct extends PointerType {
    public DummyStruct() {
      super(null);
    }
    public DummyStruct(Pointer p) {
      super(p);
    }
  }

  class DummyStructByReference extends DummyStruct {
    public DummyStructByReference() {
      super(null);
    }
    public DummyStructByReference(Pointer p) {
      super(p);
    }
  }

  class EnumWithAssociatedConstantInImpl extends PointerType {
    public EnumWithAssociatedConstantInImpl() {
      super(null);
    }
    public EnumWithAssociatedConstantInImpl(Pointer p) {
      super(p);
    }
  }

  class EnumWithAssociatedConstantInImplByReference extends EnumWithAssociatedConstantInImpl {
    public EnumWithAssociatedConstantInImplByReference() {
      super(null);
    }
    public EnumWithAssociatedConstantInImplByReference(Pointer p) {
      super(p);
    }
  }

  class TransparentComplexWrappingStructTuple extends DummyStruct {
    public TransparentComplexWrappingStructTuple() {
      super();
    }
    public TransparentComplexWrappingStructTuple(Pointer p) {
      super(p);
    }
  }

  class TransparentComplexWrappingStructTupleByReference extends DummyStructByReference {
    public TransparentComplexWrappingStructTupleByReference() {
      super();
    }
    public TransparentComplexWrappingStructTupleByReference(Pointer p) {
      super(p);
    }
  }


  class TransparentPrimitiveWrappingStructTuple extends IntegerType {
    public TransparentPrimitiveWrappingStructTuple() {
      super(4, true);
    }

    public TransparentPrimitiveWrappingStructTuple(long value) {
      super(4, value, true);
    }

    public TransparentPrimitiveWrappingStructTuple(Pointer p) {
      this(p.getInt(0));
    }

  }

  class TransparentPrimitiveWrappingStructTupleByReference extends ByReference {
    public TransparentPrimitiveWrappingStructTupleByReference() {
      super(4);
    }

    public TransparentPrimitiveWrappingStructTupleByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public TransparentPrimitiveWrappingStructTuple getValue() {
      Pointer p = getPointer();
      return new TransparentPrimitiveWrappingStructTuple(p.getInt(0));
    }

    public void setValue(TransparentPrimitiveWrappingStructTuple value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }


  class TransparentComplexWrappingStructure extends DummyStruct {
    public TransparentComplexWrappingStructure() {
      super();
    }
    public TransparentComplexWrappingStructure(Pointer p) {
      super(p);
    }
  }

  class TransparentComplexWrappingStructureByReference extends DummyStructByReference {
    public TransparentComplexWrappingStructureByReference() {
      super();
    }
    public TransparentComplexWrappingStructureByReference(Pointer p) {
      super(p);
    }
  }


  class TransparentPrimitiveWrappingStructure extends IntegerType {
    public TransparentPrimitiveWrappingStructure() {
      super(4, true);
    }

    public TransparentPrimitiveWrappingStructure(long value) {
      super(4, value, true);
    }

    public TransparentPrimitiveWrappingStructure(Pointer p) {
      this(p.getInt(0));
    }

  }

  class TransparentPrimitiveWrappingStructureByReference extends ByReference {
    public TransparentPrimitiveWrappingStructureByReference() {
      super(4);
    }

    public TransparentPrimitiveWrappingStructureByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public TransparentPrimitiveWrappingStructure getValue() {
      Pointer p = getPointer();
      return new TransparentPrimitiveWrappingStructure(p.getInt(0));
    }

    public void setValue(TransparentPrimitiveWrappingStructure value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }


  class TransparentComplexWrapper_i32 extends DummyStruct {
    public TransparentComplexWrapper_i32() {
      super();
    }
    public TransparentComplexWrapper_i32(Pointer p) {
      super(p);
    }
  }

  class TransparentComplexWrapper_i32ByReference extends DummyStructByReference {
    public TransparentComplexWrapper_i32ByReference() {
      super();
    }
    public TransparentComplexWrapper_i32ByReference(Pointer p) {
      super(p);
    }
  }


  class TransparentPrimitiveWrapper_i32 extends IntegerType {
    public TransparentPrimitiveWrapper_i32() {
      super(4, true);
    }

    public TransparentPrimitiveWrapper_i32(long value) {
      super(4, value, true);
    }

    public TransparentPrimitiveWrapper_i32(Pointer p) {
      this(p.getInt(0));
    }

  }

  class TransparentPrimitiveWrapper_i32ByReference extends ByReference {
    public TransparentPrimitiveWrapper_i32ByReference() {
      super(4);
    }

    public TransparentPrimitiveWrapper_i32ByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public TransparentPrimitiveWrapper_i32 getValue() {
      Pointer p = getPointer();
      return new TransparentPrimitiveWrapper_i32(p.getInt(0));
    }

    public void setValue(TransparentPrimitiveWrapper_i32 value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class TransparentPrimitiveWithAssociatedConstants extends IntegerType {
    public TransparentPrimitiveWithAssociatedConstants() {
      super(4, true);
    }

    public TransparentPrimitiveWithAssociatedConstants(long value) {
      super(4, value, true);
    }

    public TransparentPrimitiveWithAssociatedConstants(Pointer p) {
      this(p.getInt(0));
    }

  }

  class TransparentPrimitiveWithAssociatedConstantsByReference extends ByReference {
    public TransparentPrimitiveWithAssociatedConstantsByReference() {
      super(4);
    }

    public TransparentPrimitiveWithAssociatedConstantsByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public TransparentPrimitiveWithAssociatedConstants getValue() {
      Pointer p = getPointer();
      return new TransparentPrimitiveWithAssociatedConstants(p.getInt(0));
    }

    public void setValue(TransparentPrimitiveWithAssociatedConstants value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }

  public static final TransparentPrimitiveWithAssociatedConstants ZERO  = new TransparentPrimitiveWithAssociatedConstants(0);

  public static final TransparentPrimitiveWithAssociatedConstants ONE  = new TransparentPrimitiveWithAssociatedConstants(1);



  class TransparentEmptyStructure extends Structure implements Structure.ByValue {
    public TransparentEmptyStructure() {
      super();
    }

    public TransparentEmptyStructure(Pointer p) {
      super(p);
    }


  }

  class TransparentEmptyStructureByReference extends Structure implements Structure.ByReference {
    public TransparentEmptyStructureByReference() {
      super();
    }

    public TransparentEmptyStructureByReference(Pointer p) {
      super(p);
    }


  }


  public static final TransparentPrimitiveWrappingStructure TEN  = new TransparentPrimitiveWrappingStructure(10);


  void root(TransparentComplexWrappingStructTuple a, 
            TransparentPrimitiveWrappingStructTuple b, 
            TransparentComplexWrappingStructure c, 
            TransparentPrimitiveWrappingStructure d, 
            TransparentComplexWrapper_i32 e, 
            TransparentPrimitiveWrapper_i32 f, 
            TransparentPrimitiveWithAssociatedConstants g, 
            TransparentEmptyStructure h, 
            EnumWithAssociatedConstantInImpl i);

}