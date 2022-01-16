const std = @import("std");

const Opaque = opaque {};

pub const Foo_u64 = extern struct {
    _a: ?*f32,
    _b: ?*u64,
    _c: ?*Opaque,
    __d: ?*u64,
    __e: ?*f32,
    __f: ?*Opaque,
    _g: ?*u64,
    _h: ?*i32,
    __i: ?*i32,
};

extern fn root(_arg: ?*i32, _foo: ?*Foo_u64, __d: ?*Opaque) anyopaque;
