const std = @import("std");

pub const Opaque = opaque {};

pub const SelfTypeTestStruct = extern struct {
    times: u8,
};

pub const PointerToOpaque = extern struct {
    ptr: ?*Opaque,
};

pub extern fn rust_print_hello_world() anyopaque;

pub extern fn SelfTypeTestStruct_should_exist_ref(self: [*]const SelfTypeTestStruct) anyopaque;

pub extern fn SelfTypeTestStruct_should_exist_ref_mut(self: [*]SelfTypeTestStruct) anyopaque;

pub extern fn SelfTypeTestStruct_should_not_exist_box(self: [*]SelfTypeTestStruct) anyopaque;

pub extern fn SelfTypeTestStruct_should_not_exist_return_box() [*]SelfTypeTestStruct;

pub extern fn SelfTypeTestStruct_should_exist_annotated_self(self: SelfTypeTestStruct) anyopaque;

pub extern fn SelfTypeTestStruct_should_exist_annotated_mut_self(self: SelfTypeTestStruct) anyopaque;

pub extern fn SelfTypeTestStruct_should_exist_annotated_by_name(self: SelfTypeTestStruct) anyopaque;

pub extern fn SelfTypeTestStruct_should_exist_annotated_mut_by_name(self: SelfTypeTestStruct) anyopaque;

pub extern fn SelfTypeTestStruct_should_exist_unannotated(self: SelfTypeTestStruct) anyopaque;

pub extern fn SelfTypeTestStruct_should_exist_mut_unannotated(self: SelfTypeTestStruct) anyopaque;

pub extern fn free_function_should_exist_ref(test_struct: [*]const SelfTypeTestStruct) anyopaque;

pub extern fn free_function_should_exist_ref_mut(test_struct: [*]SelfTypeTestStruct) anyopaque;

pub extern fn unnamed_argument([*]SelfTypeTestStruct) anyopaque;

pub extern fn free_function_should_not_exist_box(boxed: [*]SelfTypeTestStruct) anyopaque;

pub extern fn free_function_should_exist_annotated_by_name(test_struct: SelfTypeTestStruct) anyopaque;

pub extern fn free_function_should_exist_annotated_mut_by_name(test_struct: SelfTypeTestStruct) anyopaque;

pub extern fn PointerToOpaque_create(times: u8) PointerToOpaque;

pub extern fn PointerToOpaque_sayHello(self: PointerToOpaque) anyopaque;
