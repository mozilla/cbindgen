const std = @import("std");

pub const AlignFlags = extern struct {
    bits: u8,
};

pub const DebugFlags = extern struct {
    bits: u32,
};

extern fn root(flags: AlignFlags, bigger_flags: DebugFlags) void;

pub const AlignFlags_AUTO = std.mem.zeroInit(AlignFlags, .{
    .bits = std.zig.c_translation.cast(u8, @as(c_int, 0)),
});

pub const AlignFlags_NORMAL = std.mem.zeroInit(AlignFlags, .{
    .bits = std.zig.c_translation.cast(u8, @as(c_int, 1)),
});

pub const AlignFlags_START = std.mem.zeroInit(AlignFlags, .{
    .bits = std.zig.c_translation.cast(u8, @as(c_int, 1) << @as(c_int, 1)),
});

pub const AlignFlags_END = std.mem.zeroInit(AlignFlags, .{
    .bits = std.zig.c_translation.cast(u8, @as(c_int, 1) << @as(c_int, 2)),
});

pub const AlignFlags_FLEX_START = std.mem.zeroInit(AlignFlags, .{
    .bits = std.zig.c_translation.cast(u8, @as(c_int, 1) << @as(c_int, 3)),
});

pub const DebugFlags_BIGGEST_ALLOWED = std.mem.zeroInit(DebugFlags, .{
    .bits = std.zig.c_translation.cast(u32, @as(c_int, 1) << @as(c_int, 31)),
});
