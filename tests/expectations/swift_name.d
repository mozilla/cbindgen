module cbindgen;

@nogc nothrow @safe:

struct Opaque;

struct SelfTypeTestStruct {
  @disable this();
  ubyte times;
}

struct PointerToOpaque {
  @disable this();
  Opaque *ptr;
}

extern(C) {

void rust_print_hello_world();

void SelfTypeTestStruct_should_exist_ref(const SelfTypeTestStruct *self);

void SelfTypeTestStruct_should_exist_ref_mut(SelfTypeTestStruct *self);

void SelfTypeTestStruct_should_not_exist_box(SelfTypeTestStruct *self);

SelfTypeTestStruct *SelfTypeTestStruct_should_not_exist_return_box();

void SelfTypeTestStruct_should_exist_annotated_self(SelfTypeTestStruct self);

void SelfTypeTestStruct_should_exist_annotated_mut_self(SelfTypeTestStruct self);

void SelfTypeTestStruct_should_exist_annotated_by_name(SelfTypeTestStruct self);

void SelfTypeTestStruct_should_exist_annotated_mut_by_name(SelfTypeTestStruct self);

void SelfTypeTestStruct_should_exist_unannotated(SelfTypeTestStruct self);

void SelfTypeTestStruct_should_exist_mut_unannotated(SelfTypeTestStruct self);

void free_function_should_exist_ref(const SelfTypeTestStruct *test_struct);

void free_function_should_exist_ref_mut(SelfTypeTestStruct *test_struct);

void unnamed_argument(SelfTypeTestStruct*);

void free_function_should_not_exist_box(SelfTypeTestStruct *boxed);

void free_function_should_exist_annotated_by_name(SelfTypeTestStruct test_struct);

void free_function_should_exist_annotated_mut_by_name(SelfTypeTestStruct test_struct);

PointerToOpaque PointerToOpaque_create(ubyte times);

void PointerToOpaque_sayHello(PointerToOpaque self);

}  // extern(C)
