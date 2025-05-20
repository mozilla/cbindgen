const std = @import("std");

pub const FOO: u32 = 10;

pub const Foo = extern struct {
    x: [FOO]i32,
};

extern fn root(x: Foo) anyopaque;
