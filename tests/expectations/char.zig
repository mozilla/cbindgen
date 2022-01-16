const std = @import("std");

pub const Foo = extern struct {
    a: u32,
};
extern fn root(a: Foo) anyopaque;
