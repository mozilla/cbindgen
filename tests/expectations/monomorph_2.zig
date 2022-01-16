const std = @import("std");

const A = opaque {};

const B = opaque {};

pub const List_A = extern struct {
    _members: ?*A,
    count: usize,
};

pub const List_B = extern struct {
    _members: ?*B,
    count: usize,
};

extern fn foo(a: List_A) anyopaque;

extern fn bar(b: List_B) anyopaque;
