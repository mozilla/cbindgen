const std = @import("std");

const Opaque = opaque {};

pub const Normal = extern union {
    x: i32,
    y: f32,
};

pub const NormalWithZST = extern union {
    x: i32,
    y: f32,
};

pub extern fn root(a: ?*Opaque, b: Normal, c: NormalWithZST) anyopaque;
