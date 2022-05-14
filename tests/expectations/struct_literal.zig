pub const Bar = opaque {};
pub const Foo = extern struct {
    a: i32,
    b: u32,
};

pub const Foo_FOO = @import("std").mem.zeroInit(Foo, .{
    .a = @as(c_int, 42),
    .b = @as(c_int, 47),
});
pub const Foo_FOO2 = @import("std").mem.zeroInit(Foo, .{
    .a = @as(c_int, 42),
    .b = @as(c_int, 47),
});
pub const Foo_FOO3 = @import("std").mem.zeroInit(Foo, .{
    .a = @as(c_int, 42),
    .b = @as(c_int, 47),
});
pub const BAR = @import("std").mem.zeroInit(Foo, .{
    .a = @as(c_int, 42),
    .b = @as(c_int, 1337),
});

pub extern fn root(x: Foo, bar: Bar) void;
