const std = @import("std");

pub const Opaque = opaque {};
pub const Option_____Opaque = opaque {};

pub const Foo = extern struct {
    x: ?*const Opaque,
    y: ?*Opaque,
    z: ?*const fn () anyopaque,
    zz: [*]?*const fn () anyopaque,
};

pub const Bar = extern union {
    x: ?*const Opaque,
    y: ?*Opaque,
    z: ?*const fn () anyopaque,
    zz: [*]?*const fn () anyopaque,
};

pub extern fn root(a: ?*const Opaque, b: ?*Opaque, c: Foo, d: Bar, e: ?*Option_____Opaque, f: ?*const fn (?*const Opaque) anyopaque) anyopaque;
