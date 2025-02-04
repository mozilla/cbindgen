const std = @import("std");

pub const MyCallback =  : ?fn(a: usize, b: usize) anyopaque;

pub const MyOtherCallback =  : ?fn(a: usize,
                                   lot: usize,
                                   of: usize,
                                   args: usize,
                                   and_then_some: usize) anyopaque;

pub extern fn my_function(a: MyCallback, b: MyOtherCallback) callconv(.C) void;
