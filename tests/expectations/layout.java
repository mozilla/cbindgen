
import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class RustAlign4Struct extends PointerType {
    public RustAlign4Struct() {
      super(null);
    }
    public RustAlign4Struct(Pointer p) {
      super(p);
    }
  }

  class RustAlign4StructByReference extends RustAlign4Struct {
    public RustAlign4StructByReference() {
      super(null);
    }
    public RustAlign4StructByReference(Pointer p) {
      super(p);
    }
  }

  class RustAlign4Union extends PointerType {
    public RustAlign4Union() {
      super(null);
    }
    public RustAlign4Union(Pointer p) {
      super(p);
    }
  }

  class RustAlign4UnionByReference extends RustAlign4Union {
    public RustAlign4UnionByReference() {
      super(null);
    }
    public RustAlign4UnionByReference(Pointer p) {
      super(p);
    }
  }

  class RustPackedStruct extends PointerType {
    public RustPackedStruct() {
      super(null);
    }
    public RustPackedStruct(Pointer p) {
      super(p);
    }
  }

  class RustPackedStructByReference extends RustPackedStruct {
    public RustPackedStructByReference() {
      super(null);
    }
    public RustPackedStructByReference(Pointer p) {
      super(p);
    }
  }

  class RustPackedUnion extends PointerType {
    public RustPackedUnion() {
      super(null);
    }
    public RustPackedUnion(Pointer p) {
      super(p);
    }
  }

  class RustPackedUnionByReference extends RustPackedUnion {
    public RustPackedUnionByReference() {
      super(null);
    }
    public RustPackedUnionByReference(Pointer p) {
      super(p);
    }
  }

  class UnsupportedAlign4Enum extends PointerType {
    public UnsupportedAlign4Enum() {
      super(null);
    }
    public UnsupportedAlign4Enum(Pointer p) {
      super(p);
    }
  }

  class UnsupportedAlign4EnumByReference extends UnsupportedAlign4Enum {
    public UnsupportedAlign4EnumByReference() {
      super(null);
    }
    public UnsupportedAlign4EnumByReference(Pointer p) {
      super(p);
    }
  }

  class UnsupportedPacked4Struct extends PointerType {
    public UnsupportedPacked4Struct() {
      super(null);
    }
    public UnsupportedPacked4Struct(Pointer p) {
      super(p);
    }
  }

  class UnsupportedPacked4StructByReference extends UnsupportedPacked4Struct {
    public UnsupportedPacked4StructByReference() {
      super(null);
    }
    public UnsupportedPacked4StructByReference(Pointer p) {
      super(p);
    }
  }

  class UnsupportedPacked4Union extends PointerType {
    public UnsupportedPacked4Union() {
      super(null);
    }
    public UnsupportedPacked4Union(Pointer p) {
      super(p);
    }
  }

  class UnsupportedPacked4UnionByReference extends UnsupportedPacked4Union {
    public UnsupportedPacked4UnionByReference() {
      super(null);
    }
    public UnsupportedPacked4UnionByReference(Pointer p) {
      super(p);
    }
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



  @Structure.FieldOrder({"arg1", "arg2"})
  class PackedStruct extends Structure implements Structure.ByValue {
    public PackedStruct() {
      super();
    }

    public PackedStruct(Pointer p) {
      super(p);
    }

    public _Size arg1;
    public ByteByReference arg2;

  }

  @Structure.FieldOrder({"arg1", "arg2"})
  class PackedStructByReference extends Structure implements Structure.ByReference {
    public PackedStructByReference() {
      super();
    }

    public PackedStructByReference(Pointer p) {
      super(p);
    }

    public _Size arg1;
    public ByteByReference arg2;

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



  @Structure.FieldOrder({"variant1", "variant2"})
  class PackedUnion extends Union implements Structure.ByValue {
    public PackedUnion() {
      super();
    }

    public PackedUnion(Pointer p) {
      super(p);
    }

    public _Size variant1;
    public ByteByReference variant2;

  }

  @Structure.FieldOrder({"variant1", "variant2"})
  class PackedUnionByReference extends Union implements Structure.ByReference {
    public PackedUnionByReference() {
      super();
    }

    public PackedUnionByReference(Pointer p) {
      super(p);
    }

    public _Size variant1;
    public ByteByReference variant2;

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