const std = @import("std");

pub const MaybeOwnedPtr_i32_Tag = enum {
  Owned_i32,
  None_i32,
};

pub const MaybeOwnedPtr_i32 = extern struct {
  tag: MaybeOwnedPtr_i32_Tag,
};