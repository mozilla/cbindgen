const std = @import("std");

pub const IE = enum {
    IV,
};

pub const UE = enum {
    UV,
};

pub const Usize = usize;

pub const Isize = isize;

pub extern fn root(Usize, Isize, UE, IE) anyopaque;
