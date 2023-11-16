const std = @import("std");

pub const Rect = extern struct {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
};
pub const Color = extern struct {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
};

pub const DisplayItem_Tag = enum(c_int) {
    Fill,
    Image,
    ClearScreen,
};

pub const Fill_Body = extern struct {
    tag: DisplayItem_Tag,
    _0: Rect,
    _1: Color,
};
pub const Image_Body = extern struct {
    tag: DisplayItem_Tag,
    id: u32,
    bounds: Rect,
};
pub const DisplayItem = extern union {
    tag: DisplayItem_Tag,
    fill: Fill_Body,
    image: Image_Body,
};

extern fn push_item(item: DisplayItem) bool;
