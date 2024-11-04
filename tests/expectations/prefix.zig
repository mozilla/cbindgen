const std = @import("std");

pub const LEN = 22;

pub const  X = (22 << 22);

pub const  Y = (X + X);

pub const NamedLenArray = [LEN]i32;

pub const ValuedLenArray = [22]i32;

pub const AbsoluteFontWeight_Tag = enum {
  Weight,
  Normal,
  Bold,
};

pub const AbsoluteFontWeight = extern union {
  tag: AbsoluteFontWeight_Tag,
   weight_tag: AbsoluteFontWeight_Tag,
   weight: f32,
};
