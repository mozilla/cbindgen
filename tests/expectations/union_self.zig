const std = @import("std");

pub const Foo_Bar = extern struct {
    something: ?*i32,
};

pub const Bar = extern union {
    something: i32,
    subexpressions: Foo_Bar,
};

pub extern fn root(b: Bar) anyopaque;
