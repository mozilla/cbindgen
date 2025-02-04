const std = @import("std");

pub const TraitObject = extern struct {
    data: ?*anyopaque,
    vtable: ?*anyopaque,
};

extern fn root(ptr: ?*const anyopaque, t: TraitObject) ?*anyopaque;
