const std = @import("std");

pub const Point_i32 = extern struct {
    x: i32,
    y: i32,
};

pub const Point_f32 = extern struct {
    x: f32,
    y: f32,
};

pub const Foo_i32_Tag = enum {
    Foo_i32,
    Bar_i32,
    Baz_i32,
    Bazz_i32,
};

pub const Foo_Body_i32 = extern struct {
    tag: Foo_i32_Tag,
    x: i32,
    y: Point_i32,
    z: Point_f32,
};

pub const Foo_i32 = extern union {
    tag: Foo_i32_Tag,
    foo: Foo_Body_i32,
};

pub const Bar_i32_Tag = enum {
    Bar1_i32,
    Bar2_i32,
    Bar3_i32,
    Bar4_i32,
};

pub const Bar1_Body_i32 = extern struct {
    x: i32,
    y: Point_i32,
    z: Point_f32,
    u: ?fn () i32,
};

pub const Bar_i32 = extern struct {
    tag: Bar_i32_Tag,
};

pub const Point_u32 = extern struct {
    x: u32,
    y: u32,
};

pub const Bar_u32_Tag = enum {
    Bar1_u32,
    Bar2_u32,
    Bar3_u32,
    Bar4_u32,
};

pub const Bar1_Body_u32 = extern struct {
    x: i32,
    y: Point_u32,
    z: Point_f32,
    u: ?fn () i32,
};

pub const Bar_u32 = extern struct {
    tag: Bar_u32_Tag,
};

pub const Baz_Tag = enum {
    Baz1,
    Baz2,
    Baz3,
};

pub const Baz = extern union {
    tag: Baz_Tag,
};

pub const Taz_Tag = enum {
    Taz1,
    Taz2,
    Taz3,
};

pub const Taz = extern struct {
    tag: Taz_Tag,
};

pub extern fn foo(foo: ?*Foo_i32, bar: ?*Bar_i32, baz: ?*Baz, taz: ?*Taz) anyopaque;
