
import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class OpaquePackedStruct extends PointerType {
    public OpaquePackedStruct() {
      super(null);
    }
    public OpaquePackedStruct(Pointer p) {
      super(p);
    }
  }

  class OpaquePackedStructByReference extends OpaquePackedStruct {
    public OpaquePackedStructByReference() {
      super(null);
    }
    public OpaquePackedStructByReference(Pointer p) {
      super(p);
    }
  }

  class OpaquePackedUnion extends PointerType {
    public OpaquePackedUnion() {
      super(null);
    }
    public OpaquePackedUnion(Pointer p) {
      super(p);
    }
  }

  class OpaquePackedUnionByReference extends OpaquePackedUnion {
    public OpaquePackedUnionByReference() {
      super(null);
    }
    public OpaquePackedUnionByReference(Pointer p) {
      super(p);
    }
  }


  @Structure.FieldOrder({"variant1", "variant2"})
  class Align1Union extends Union implements Structure.ByValue {
    public Align1Union() {
      super();
    }

    public Align1Union(Pointer p) {
      super(p);
    }

    public _Size variant1;
    public ByteByReference variant2;

  }

  @Structure.FieldOrder({"variant1", "variant2"})
  class Align1UnionByReference extends Union implements Structure.ByReference {
    public Align1UnionByReference() {
      super();
    }

    public Align1UnionByReference(Pointer p) {
      super(p);
    }

    public _Size variant1;
    public ByteByReference variant2;

  }



  @Structure.FieldOrder({"variant1", "variant2"})
  class Align4Union extends Union implements Structure.ByValue {
    public Align4Union() {
      super();
    }

    public Align4Union(Pointer p) {
      super(p);
    }

    public _Size variant1;
    public ByteByReference variant2;

  }

  @Structure.FieldOrder({"variant1", "variant2"})
  class Align4UnionByReference extends Union implements Structure.ByReference {
    public Align4UnionByReference() {
      super();
    }

    public Align4UnionByReference(Pointer p) {
      super(p);
    }

    public _Size variant1;
    public ByteByReference variant2;

  }



  @Structure.FieldOrder({"variant1", "variant2"})
  class Align16Union extends Union implements Structure.ByValue {
    public Align16Union() {
      super();
    }

    public Align16Union(Pointer p) {
      super(p);
    }

    public _Size variant1;
    public ByteByReference variant2;

  }

  @Structure.FieldOrder({"variant1", "variant2"})
  class Align16UnionByReference extends Union implements Structure.ByReference {
    public Align16UnionByReference() {
      super();
    }

    public Align16UnionByReference(Pointer p) {
      super(p);
    }

    public _Size variant1;
    public ByteByReference variant2;

  }



  @Structure.FieldOrder({"arg1", "arg2"})
  class Align1Struct extends Structure implements Structure.ByValue {
    public Align1Struct() {
      super();
    }

    public Align1Struct(Pointer p) {
      super(p);
    }

    public _Size arg1;
    public ByteByReference arg2;

  }

  @Structure.FieldOrder({"arg1", "arg2"})
  class Align1StructByReference extends Structure implements Structure.ByReference {
    public Align1StructByReference() {
      super();
    }

    public Align1StructByReference(Pointer p) {
      super(p);
    }

    public _Size arg1;
    public ByteByReference arg2;

  }



  @Structure.FieldOrder({"arg1", "arg2"})
  class Align2Struct extends Structure implements Structure.ByValue {
    public Align2Struct() {
      super();
    }

    public Align2Struct(Pointer p) {
      super(p);
    }

    public _Size arg1;
    public ByteByReference arg2;

  }

  @Structure.FieldOrder({"arg1", "arg2"})
  class Align2StructByReference extends Structure implements Structure.ByReference {
    public Align2StructByReference() {
      super();
    }

    public Align2StructByReference(Pointer p) {
      super(p);
    }

    public _Size arg1;
    public ByteByReference arg2;

  }



  @Structure.FieldOrder({"arg1", "arg2"})
  class Align4Struct extends Structure implements Structure.ByValue {
    public Align4Struct() {
      super();
    }

    public Align4Struct(Pointer p) {
      super(p);
    }

    public _Size arg1;
    public ByteByReference arg2;

  }

  @Structure.FieldOrder({"arg1", "arg2"})
  class Align4StructByReference extends Structure implements Structure.ByReference {
    public Align4StructByReference() {
      super();
    }

    public Align4StructByReference(Pointer p) {
      super(p);
    }

    public _Size arg1;
    public ByteByReference arg2;

  }



  @Structure.FieldOrder({"arg1", "arg2"})
  class Align8Struct extends Structure implements Structure.ByValue {
    public Align8Struct() {
      super();
    }

    public Align8Struct(Pointer p) {
      super(p);
    }

    public _Size arg1;
    public ByteByReference arg2;

  }

  @Structure.FieldOrder({"arg1", "arg2"})
  class Align8StructByReference extends Structure implements Structure.ByReference {
    public Align8StructByReference() {
      super();
    }

    public Align8StructByReference(Pointer p) {
      super(p);
    }

    public _Size arg1;
    public ByteByReference arg2;

  }



  @Structure.FieldOrder({"arg1", "arg2"})
  class Align32Struct extends Structure implements Structure.ByValue {
    public Align32Struct() {
      super();
    }

    public Align32Struct(Pointer p) {
      super(p);
    }

    public _Size arg1;
    public ByteByReference arg2;

  }

  @Structure.FieldOrder({"arg1", "arg2"})
  class Align32StructByReference extends Structure implements Structure.ByReference {
    public Align32StructByReference() {
      super();
    }

    public Align32StructByReference(Pointer p) {
      super(p);
    }

    public _Size arg1;
    public ByteByReference arg2;

  }


  class _Size extends IntegerType {
    public _Size() {
      super(Native.POINTER_SIZE, true);
    }

    public _Size(long value) {
      super(Native.POINTER_SIZE, value, true);
    }

    public _Size(Pointer p) {
      this(Native.POINTER_SIZE == 8 ? p.getLong(0) : p.getInt(0));
    }

  }

  class _SizeByReference extends ByReference {
    public _SizeByReference() {
      super(Native.POINTER_SIZE);
    }

    public _SizeByReference(Pointer p) {
      super(Native.POINTER_SIZE);
      setPointer(p);
    }

    public _Size getValue() {
      Pointer p = getPointer();
      return new _Size(Native.POINTER_SIZE == 8 ? p.getLong(0) : p.getInt(0));
    }

    public void setValue(_Size value) {
      Pointer p = getPointer();
      if (Native.POINTER_SIZE == 8) { p.setLong(0, value.longValue()); } else { p.setInt(0, value.intValue()); }
    }

  }

}