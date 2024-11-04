
import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class Opaque extends PointerType {
    public Opaque() {
      super(null);
    }
    public Opaque(Pointer p) {
      super(p);
    }
  }

  class OpaqueByReference extends Opaque {
    public OpaqueByReference() {
      super(null);
    }
    public OpaqueByReference(Pointer p) {
      super(p);
    }
  }


  @Structure.FieldOrder({"times"})
  class SelfTypeTestStruct extends Structure implements Structure.ByValue {
    public SelfTypeTestStruct() {
      super();
    }

    public SelfTypeTestStruct(Pointer p) {
      super(p);
    }

    public byte times;

  }

  @Structure.FieldOrder({"times"})
  class SelfTypeTestStructByReference extends Structure implements Structure.ByReference {
    public SelfTypeTestStructByReference() {
      super();
    }

    public SelfTypeTestStructByReference(Pointer p) {
      super(p);
    }

    public byte times;

  }



  @Structure.FieldOrder({"ptr"})
  class PointerToOpaque extends Structure implements Structure.ByValue {
    public PointerToOpaque() {
      super();
    }

    public PointerToOpaque(Pointer p) {
      super(p);
    }

    public OpaqueByReference ptr;

  }

  @Structure.FieldOrder({"ptr"})
  class PointerToOpaqueByReference extends Structure implements Structure.ByReference {
    public PointerToOpaqueByReference() {
      super();
    }

    public PointerToOpaqueByReference(Pointer p) {
      super(p);
    }

    public OpaqueByReference ptr;

  }


  void rust_print_hello_world();

  void SelfTypeTestStruct_should_exist_ref(SelfTypeTestStructByReference self);

  void SelfTypeTestStruct_should_exist_ref_mut(SelfTypeTestStructByReference self);

  void SelfTypeTestStruct_should_not_exist_box(SelfTypeTestStructByReference self);

  SelfTypeTestStructByReference SelfTypeTestStruct_should_not_exist_return_box();

  void SelfTypeTestStruct_should_exist_annotated_self(SelfTypeTestStruct self);

  void SelfTypeTestStruct_should_exist_annotated_mut_self(SelfTypeTestStruct self);

  void SelfTypeTestStruct_should_exist_annotated_by_name(SelfTypeTestStruct self);

  void SelfTypeTestStruct_should_exist_annotated_mut_by_name(SelfTypeTestStruct self);

  void SelfTypeTestStruct_should_exist_unannotated(SelfTypeTestStruct self);

  void SelfTypeTestStruct_should_exist_mut_unannotated(SelfTypeTestStruct self);

  void free_function_should_exist_ref(SelfTypeTestStructByReference test_struct);

  void free_function_should_exist_ref_mut(SelfTypeTestStructByReference test_struct);

  void unnamed_argument(SelfTypeTestStructByReference arg0);

  void free_function_should_not_exist_box(SelfTypeTestStructByReference boxed);

  void free_function_should_exist_annotated_by_name(SelfTypeTestStruct test_struct);

  void free_function_should_exist_annotated_mut_by_name(SelfTypeTestStruct test_struct);

  PointerToOpaque PointerToOpaque_create(byte times);

  void PointerToOpaque_sayHello(PointerToOpaque self);

}