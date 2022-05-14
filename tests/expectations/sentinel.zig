const std = @import("std");

pub const A = enum {
  A1,
  A2,
  A3,
};

pub const B = enum {
  B1,
  B2,
  B3,
};

pub const C_Tag = enum {
  C1,
  C2,
  C3,
};

pub const C1_Body = extern struct {
  tag: C_Tag,
  a: u32,
};

pub const C2_Body = extern struct {
  tag: C_Tag,
  b: u32,
};

pub const C = extern union {
  tag: C_Tag,
  c1: C1_Body,
  c2: C2_Body,
};

pub extern fn root(a: A, b: B, c: C) anyopaque;
