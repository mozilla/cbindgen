const std = @import("std");

pub const ExtType = extern struct {
    data: u32,
};

pub extern fn consume_ext(_ext: ExtType) anyopaque;
