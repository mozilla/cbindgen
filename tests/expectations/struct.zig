const std = @import("std");

const Opaque = opaque {

};

pub const Normal = extern struct {
  x: i32,
  y: f32,
};

pub const NormalWithZST = extern struct {
  x: i32,
  y: f32,
};

pub const TupleRenamed = extern struct {
  m0: i32,
  m1: f32,
};

pub const TupleNamed = extern struct {
  x: i32,
  y: f32,
};

pub extern fn root( a: ?*Opaque,
                   b: Normal,
                   c: NormalWithZST,
                   d: TupleRenamed,
                   e: TupleNamed) anyopaque;
