const std = @import("std");

pub const MyCLikeEnum = extern union(c_uint) {
    Foo1,
    Bar1,
    Baz1,
};

pub const MyCLikeEnum_Prepended = extern union(c_uint) {
    Foo1_Prepended,
    Bar1_Prepended,
    Baz1_Prepended,
};

pub const MyFancyStruct = extern struct {
    i: i32,
};

pub const MyFancyEnum_Tag = extern union(c_uint) {
    Foo,
    Bar,
    Baz,
};

const struct_unnamed_2 = extern struct {
    bar: i32,
};
const struct_unnamed_3 = extern struct {
    baz: i32,
};
const union_unnamed_1 = extern union {
    unnamed_0: struct_unnamed_2,
    unnamed_1: struct_unnamed_3,
};
pub const MyFancyEnum = extern struct {
    tag: MyFancyEnum_Tag,
    unnamed_0: union_unnamed_1,
};
pub const MyUnion = extern union {
    f: f32,
    u: u32,
    extra_member: i32,
};
pub const MyFancyStruct_Prepended = extern struct {
    i: i32,
};

pub const MyFancyEnum_Prepended_Tag = extern union(c_uint) {
    Foo_Prepended,
    Bar_Prepended,
    Baz_Prepended,
};

const struct_unnamed_5 = extern struct {
    bar_prepended: i32,
};
const struct_unnamed_6 = extern struct {
    baz_prepended: i32,
};
const union_unnamed_4 = extern union {
    unnamed_0: struct_unnamed_5,
    unnamed_1: struct_unnamed_6,
};
pub const MyFancyEnum_Prepended = extern struct {
    tag: MyFancyEnum_Prepended_Tag,
    unnamed_0: union_unnamed_4,
};

pub const MyUnion_Prepended = extern union {
    extra_member: i32,
    f: f32,
    u: u32,
};

extern fn root(s: MyFancyStruct, e: MyFancyEnum, c: MyCLikeEnum, u: MyUnion, sp: MyFancyStruct_Prepended, ep: MyFancyEnum_Prepended, cp: MyCLikeEnum_Prepended, up: MyUnion_Prepended) anyopaque;
