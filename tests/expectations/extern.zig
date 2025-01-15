const std = @import("std");

pub const Normal = extern struct {
  x: i32,
  y: f32,
};

extern fn foo() i32;

extern fn bar(a: Normal) anyopaque;
