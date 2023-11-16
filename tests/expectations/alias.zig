const std = @import("std");

pub const Status = enum(c_int) {
    Ok = 0,
    Err = -1,
};

pub const Dep = extern struct {
    a: i32,
    b: f32,
};

pub const Foo_i32 = extern struct {
    a: i32,
    b: i32,
    c: Dep,
};

pub const IntFoo = Foo_i32;

pub const Foo_f64 = extern struct {
    a: f64,
    b: f64,
    c: Dep,
};

pub const DoubleFoo = Foo_f64;
pub const Unit = i32;
pub const SpecialStatus = Status;

extern fn root(x: IntFoo, y: DoubleFoo, z: Unit, w: SpecialStatus) anyopaque;
