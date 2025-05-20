const std = @import("std");

const Option_i64 = opaque {};

pub const NonZeroTest = extern struct {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
    e: i8,
    f: i16,
    g: i32,
    h: i64,
    i: i64,
    j: ?*const Option_i64,
};

extern fn root(_test: NonZeroTest, a: u8, b: u16, c: u32, d: u64, e: i8, f: i16, g: i32, h: i64, i: i64, j: ?*const Option_i64) anyopaque;
