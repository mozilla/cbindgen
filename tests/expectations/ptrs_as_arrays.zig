const std = @import("std");

extern fn ptr_as_array(n: u32, arg: [3]u32, _v: ?*u64) anyopaque;

extern fn ptr_as_array1(n: u32, arg: [3]u32, v: [4]u64) anyopaque;

extern fn ptr_as_array2(n: u32, arg: [*]u32, v: [*]u64) anyopaque;

extern fn ptr_as_array_wrong_syntax(_arg: ?*u32, _v: ?*u32, _: ?*u32) anyopaque;

extern fn ptr_as_array_unnamed(_: ?*u32, _: ?*u32) anyopaque;
