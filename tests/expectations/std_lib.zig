const std = @import("std");

const Option_i32 = opaque {};

const Result_i32__String = opaque {};

const Vec_String = opaque {};

pub extern fn root(a: ?*Vec_String, b: ?*Option_i32, c: ?*Result_i32__String) anyopaque;
