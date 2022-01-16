const std = @import("std");

pub const dep_struct = extern struct {
    x: u32,
    y: f64,
};

extern fn get_x(dep_struct: ?*const dep_struct) c_int;
