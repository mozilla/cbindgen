const std = @import("std");

pub const Foo_Bar = extern struct {
   something: ?*i32,
};

pub const Bar_Tag = enum {
  Min,
  Max,
  Other,
};

pub const Bar = extern union {
  tag: Bar_Tag,
  min_tag: Bar_Tag,
  min: Foo_Bar,
  max_tag: Bar_Tag,
  max: Foo_Bar,
};

pub extern fn root(b: Bar) anyopaque;
