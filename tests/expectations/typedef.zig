const std = @import("std");

pub const Foo_i32__i32 = extern struct {
  x: i32,
  y: i32,
};

pub const IntFoo_i32 = Foo_i32__i32;

extern fn root(a: IntFoo_i32) anyopaque;
