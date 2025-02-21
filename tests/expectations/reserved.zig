const std = @import("std");

pub const A = extern struct {
   namespace_: i32,
   float_: f32,
};

pub const B = extern struct {
   namespace_: i32,
   float_: f32,
};

pub const C_Tag = enum {
  D,
};

pub const D_Body = extern struct {
   namespace_: i32,
   float_: f32,
};

pub const C = extern struct {
  tag: C_Tag,
};

pub const E_Tag = enum {
  Double,
  Float,
};

pub const E = extern struct {
  tag: E_Tag,
   float_: f32,
};