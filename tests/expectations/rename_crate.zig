const std = @import("std");

pub const Foo = extern struct {
    x: i32,
};
pub const RenamedTy = extern struct {
    y: u64,
};
pub const NoExternTy = extern struct {
    field: u8,
};
pub const ContainsNoExternTy = extern struct {
    field: NoExternTy,
};

pub extern fn root(a: Foo) anyopaque;

pub extern fn renamed_func(a: RenamedTy) anyopaque;

pub extern fn no_extern_func(a: ContainsNoExternTy) anyopaque;
