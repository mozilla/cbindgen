const std = @import("std");

pub const HasBitfields = packed struct {
    foo: u8,
    bar: u56,
};

extern fn root(?*const HasBitfields) anyopaque;
