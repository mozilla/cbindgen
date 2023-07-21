import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class Bar_Bar_f32 extends PointerType {
    public Bar_Bar_f32() {
      super(null);
    }
    public Bar_Bar_f32(Pointer p) {
      super(p);
    }
  }

  class Bar_Bar_f32ByReference extends Bar_Bar_f32 {
    public Bar_Bar_f32ByReference() {
      super(null);
    }
    public Bar_Bar_f32ByReference(Pointer p) {
      super(p);
    }
  }

  class Bar_Foo_f32 extends PointerType {
    public Bar_Foo_f32() {
      super(null);
    }
    public Bar_Foo_f32(Pointer p) {
      super(p);
    }
  }

  class Bar_Foo_f32ByReference extends Bar_Foo_f32 {
    public Bar_Foo_f32ByReference() {
      super(null);
    }
    public Bar_Foo_f32ByReference(Pointer p) {
      super(p);
    }
  }

  class Bar_f32 extends PointerType {
    public Bar_f32() {
      super(null);
    }
    public Bar_f32(Pointer p) {
      super(p);
    }
  }

  class Bar_f32ByReference extends Bar_f32 {
    public Bar_f32ByReference() {
      super(null);
    }
    public Bar_f32ByReference(Pointer p) {
      super(p);
    }
  }


  @Structure.FieldOrder({"data"})
  class Foo_i32 extends Union implements Structure.ByValue {
    public Foo_i32() {
      super();
    }

    public Foo_i32(Pointer p) {
      super(p);
    }

    public IntByReference data;

  }

  @Structure.FieldOrder({"data"})
  class Foo_i32ByReference extends Union implements Structure.ByReference {
    public Foo_i32ByReference() {
      super();
    }

    public Foo_i32ByReference(Pointer p) {
      super(p);
    }

    public IntByReference data;

  }



  @Structure.FieldOrder({"data"})
  class Foo_f32 extends Union implements Structure.ByValue {
    public Foo_f32() {
      super();
    }

    public Foo_f32(Pointer p) {
      super(p);
    }

    public FloatByReference data;

  }

  @Structure.FieldOrder({"data"})
  class Foo_f32ByReference extends Union implements Structure.ByReference {
    public Foo_f32ByReference() {
      super();
    }

    public Foo_f32ByReference(Pointer p) {
      super(p);
    }

    public FloatByReference data;

  }



  @Structure.FieldOrder({"data"})
  class Foo_Bar_f32 extends Union implements Structure.ByValue {
    public Foo_Bar_f32() {
      super();
    }

    public Foo_Bar_f32(Pointer p) {
      super(p);
    }

    public Bar_f32ByReference data;

  }

  @Structure.FieldOrder({"data"})
  class Foo_Bar_f32ByReference extends Union implements Structure.ByReference {
    public Foo_Bar_f32ByReference() {
      super();
    }

    public Foo_Bar_f32ByReference(Pointer p) {
      super(p);
    }

    public Bar_f32ByReference data;

  }



  @Structure.FieldOrder({"a", "b"})
  class Tuple_Foo_f32_____f32 extends Union implements Structure.ByValue {
    public Tuple_Foo_f32_____f32() {
      super();
    }

    public Tuple_Foo_f32_____f32(Pointer p) {
      super(p);
    }

    public Foo_f32ByReference a;
    public FloatByReference b;

  }

  @Structure.FieldOrder({"a", "b"})
  class Tuple_Foo_f32_____f32ByReference extends Union implements Structure.ByReference {
    public Tuple_Foo_f32_____f32ByReference() {
      super();
    }

    public Tuple_Foo_f32_____f32ByReference(Pointer p) {
      super(p);
    }

    public Foo_f32ByReference a;
    public FloatByReference b;

  }



  @Structure.FieldOrder({"a", "b"})
  class Tuple_f32__f32 extends Union implements Structure.ByValue {
    public Tuple_f32__f32() {
      super();
    }

    public Tuple_f32__f32(Pointer p) {
      super(p);
    }

    public FloatByReference a;
    public FloatByReference b;

  }

  @Structure.FieldOrder({"a", "b"})
  class Tuple_f32__f32ByReference extends Union implements Structure.ByReference {
    public Tuple_f32__f32ByReference() {
      super();
    }

    public Tuple_f32__f32ByReference(Pointer p) {
      super(p);
    }

    public FloatByReference a;
    public FloatByReference b;

  }


  class Indirection_f32 extends Tuple_f32__f32 {
    public Indirection_f32() {
      super();
    }
    public Indirection_f32(Pointer p) {
      super(p);
    }
  }

  class Indirection_f32ByReference extends Tuple_f32__f32ByReference {
    public Indirection_f32ByReference() {
      super();
    }
    public Indirection_f32ByReference(Pointer p) {
      super(p);
    }
  }

  void root(Foo_i32 a, 
            Foo_f32 b, 
            Bar_f32 c, 
            Foo_Bar_f32 d, 
            Bar_Foo_f32 e, 
            Bar_Bar_f32 f, 
            Tuple_Foo_f32_____f32 g, 
            Indirection_f32 h);

}