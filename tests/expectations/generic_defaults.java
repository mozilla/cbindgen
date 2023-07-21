import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class Foo_i16 extends IntegerType {
    public Foo_i16() {
      super(2);
    }

    public Foo_i16(long value) {
      super(2, value);
    }

    public Foo_i16(Pointer p) {
      this(p.getShort(0));
    }

  }

  class Foo_i16ByReference extends ByReference {
    public Foo_i16ByReference() {
      super(2);
    }

    public Foo_i16ByReference(Pointer p) {
      super(2);
      setPointer(p);
    }

    public Foo_i16 getValue() {
      return new Foo_i16(getPointer().getShort(0));
    }

    public void setValue(Foo_i16 value) {
      getPointer().setShort(0, (short)value.intValue());
    }

  }

  class Foo_i32 extends IntegerType {
    public Foo_i32() {
      super(4);
    }

    public Foo_i32(long value) {
      super(4, value);
    }

    public Foo_i32(Pointer p) {
      this(p.getInt(0));
    }

  }

  class Foo_i32ByReference extends ByReference {
    public Foo_i32ByReference() {
      super(4);
    }

    public Foo_i32ByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public Foo_i32 getValue() {
      return new Foo_i32(getPointer().getInt(0));
    }

    public void setValue(Foo_i32 value) {
      getPointer().setInt(0, value.intValue());
    }

  }


  @Structure.FieldOrder({"f", "p"})
  class Bar_i32__u32 extends Structure implements Structure.ByValue {
    public Bar_i32__u32() {
      super();
    }

    public Bar_i32__u32(Pointer p) {
      super(p);
    }

    public Foo_i32 f;
    public int p;

  }

  @Structure.FieldOrder({"f", "p"})
  class Bar_i32__u32ByReference extends Structure implements Structure.ByReference {
    public Bar_i32__u32ByReference() {
      super();
    }

    public Bar_i32__u32ByReference(Pointer p) {
      super(p);
    }

    public Foo_i32 f;
    public int p;

  }


  class Foo_i64 extends IntegerType {
    public Foo_i64() {
      super(8);
    }

    public Foo_i64(long value) {
      super(8, value);
    }

    public Foo_i64(Pointer p) {
      this(p.getLong(0));
    }

  }

  class Foo_i64ByReference extends ByReference {
    public Foo_i64ByReference() {
      super(8);
    }

    public Foo_i64ByReference(Pointer p) {
      super(8);
      setPointer(p);
    }

    public Foo_i64 getValue() {
      return new Foo_i64(getPointer().getLong(0));
    }

    public void setValue(Foo_i64 value) {
      getPointer().setLong(0, value.longValue());
    }

  }

  class Baz_i64 extends Foo_i64 {
    public Baz_i64() {
      super();
    }
    public Baz_i64(Pointer p) {
      super(p);
    }
  }

  class Baz_i64ByReference extends Foo_i64ByReference {
    public Baz_i64ByReference() {
      super();
    }
    public Baz_i64ByReference(Pointer p) {
      super(p);
    }
  }

  void foo_root(Foo_i16 f, Bar_i32__u32 b, Baz_i64 z);

}