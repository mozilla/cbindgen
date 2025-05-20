const std = @import("std");

pub const struct_NotReprC_____i32 = opaque {};
pub const NotReprC_____i32 = struct_NotReprC_____i32;
pub const Foo = NotReprC_____i32;
pub const MyStruct = extern struct {
    number: [*c]i32,
};
extern fn root(a: ?*const Foo, with_box: [*c]const MyStruct) anyopaque;
extern fn drop_box(x: [*c]i32) anyopaque;
extern fn drop_box_opt(x: [*c]i32) anyopaque;
