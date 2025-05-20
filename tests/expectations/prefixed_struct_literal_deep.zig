const std = @import("std");

pub const PREFIXBar = extern struct {
    a: i32,
};
pub const PREFIXFoo = extern struct {
    a: i32,
    b: u32,
    bar: PREFIXBar,
};

pub extern fn root(x: PREFIXFoo) anyopaque;

pub const PREFIXVAL = @import("std").mem.zeroInit(PREFIXFoo, .{
    .a = @as(c_int, 42),
    .b = @as(c_int, 1337),
    .bar = @import("std").mem.zeroInit(PREFIXBar, .{
        .a = @as(c_int, 323),
    }),
});
