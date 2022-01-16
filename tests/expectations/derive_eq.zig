const std = @import("std");

pub const Foo = extern struct {
    a: bool,
    b: i32,
};

pub const Bar_Tag = enum {
    Baz,
    Bazz,
    FooNamed,
    FooParen,
};

pub const Bazz_Body = extern struct {
    tag: Bar_Tag,
    named: Foo,
};

pub const FooNamed_Body = extern struct {
    tag: Bar_Tag,
    different: i32,
    fields: u32,
};

pub const FooParen_Body = extern struct {
    tag: Bar_Tag,
    _0: i32,
    _1: Foo,
};

pub const Bar = extern union {
    tag: Bar_Tag,
    bazz: Bazz_Body,
    foo_named: FooNamed_Body,
    foo_paren: FooParen_Body,
};

extern fn root(bar: Bar) Foo;
