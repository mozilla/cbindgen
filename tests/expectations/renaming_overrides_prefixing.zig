const std = @import("std");

const A = opaque {

};

pub const B = extern struct {
  x: i32,
  y: f32,
};

pub extern fn root( a: ?*A, b: B) anyopaque;
