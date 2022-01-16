const std = @import("std");

pub const PinTest = extern struct {
    _pinned_box: ?*i32,
    _pinned_ref: ?*i32,
};

extern fn root(_s: ?*i32, p: PinTest) anyopaque;
