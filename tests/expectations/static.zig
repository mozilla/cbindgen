const std = @import("std");

const Bar = opaque {};

pub const Foo = extern struct {};

pub extern const NUMBER: i32;

pub extern const FOO: Foo;

pub extern const BAR: Bar;

pub extern fn root() anyopaque;
