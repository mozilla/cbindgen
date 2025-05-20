
import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class OpaqueAlign16Union extends PointerType {
    public OpaqueAlign16Union() {
      super(null);
    }
    public OpaqueAlign16Union(Pointer p) {
      super(p);
    }
  }

  class OpaqueAlign16UnionByReference extends OpaqueAlign16Union {
    public OpaqueAlign16UnionByReference() {
      super(null);
    }
    public OpaqueAlign16UnionByReference(Pointer p) {
      super(p);
    }
  }

  class OpaqueAlign1Struct extends PointerType {
    public OpaqueAlign1Struct() {
      super(null);
    }
    public OpaqueAlign1Struct(Pointer p) {
      super(p);
    }
  }

  class OpaqueAlign1StructByReference extends OpaqueAlign1Struct {
    public OpaqueAlign1StructByReference() {
      super(null);
    }
    public OpaqueAlign1StructByReference(Pointer p) {
      super(p);
    }
  }

  class OpaqueAlign1Union extends PointerType {
    public OpaqueAlign1Union() {
      super(null);
    }
    public OpaqueAlign1Union(Pointer p) {
      super(p);
    }
  }

  class OpaqueAlign1UnionByReference extends OpaqueAlign1Union {
    public OpaqueAlign1UnionByReference() {
      super(null);
    }
    public OpaqueAlign1UnionByReference(Pointer p) {
      super(p);
    }
  }

  class OpaqueAlign2Struct extends PointerType {
    public OpaqueAlign2Struct() {
      super(null);
    }
    public OpaqueAlign2Struct(Pointer p) {
      super(p);
    }
  }

  class OpaqueAlign2StructByReference extends OpaqueAlign2Struct {
    public OpaqueAlign2StructByReference() {
      super(null);
    }
    public OpaqueAlign2StructByReference(Pointer p) {
      super(p);
    }
  }

  class OpaqueAlign32Struct extends PointerType {
    public OpaqueAlign32Struct() {
      super(null);
    }
    public OpaqueAlign32Struct(Pointer p) {
      super(p);
    }
  }

  class OpaqueAlign32StructByReference extends OpaqueAlign32Struct {
    public OpaqueAlign32StructByReference() {
      super(null);
    }
    public OpaqueAlign32StructByReference(Pointer p) {
      super(p);
    }
  }

  class OpaqueAlign4Struct extends PointerType {
    public OpaqueAlign4Struct() {
      super(null);
    }
    public OpaqueAlign4Struct(Pointer p) {
      super(p);
    }
  }

  class OpaqueAlign4StructByReference extends OpaqueAlign4Struct {
    public OpaqueAlign4StructByReference() {
      super(null);
    }
    public OpaqueAlign4StructByReference(Pointer p) {
      super(p);
    }
  }

  class OpaqueAlign4Union extends PointerType {
    public OpaqueAlign4Union() {
      super(null);
    }
    public OpaqueAlign4Union(Pointer p) {
      super(p);
    }
  }

  class OpaqueAlign4UnionByReference extends OpaqueAlign4Union {
    public OpaqueAlign4UnionByReference() {
      super(null);
    }
    public OpaqueAlign4UnionByReference(Pointer p) {
      super(p);
    }
  }

  class OpaqueAlign8Struct extends PointerType {
    public OpaqueAlign8Struct() {
      super(null);
    }
    public OpaqueAlign8Struct(Pointer p) {
      super(p);
    }
  }

  class OpaqueAlign8StructByReference extends OpaqueAlign8Struct {
    public OpaqueAlign8StructByReference() {
      super(null);
    }
    public OpaqueAlign8StructByReference(Pointer p) {
      super(p);
    }
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