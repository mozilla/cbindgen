const std = @import("std");

pub const Enum = enum(c_int) {
    a,
    b,
};

pub const Struct = extern struct {
    field: Enum,
};

pub const STATIC = Enum;

extern fn (arg: Struct) anyopaque;
