const std = @import("std");

pub const A = ?fn () callconv(.C) void;
pub const B = ?fn () callconv(.C) void;
pub const C = ?fn (i32, i32) callconv(.C) bool;
pub const D = ?fn (i32) callconv(.C) ?fn (f32) callconv(.C) bool;
pub const E = ?fn () callconv(.C) [*c]const [16]i32;
pub const F = [*c]const i32;
pub const G = [*c]const [*c]const i32;
pub const H = [*c]const [*c]i32;
pub const I = [*c]const [16]i32;
pub const J = [*c]?fn (f32) callconv(.C) f64;
pub const K = [16]i32;
pub const L = [16][*c]const i32;
pub const M = [16]?fn (i32, i32) callconv(.C) bool;
pub const N = [16]?fn (i32, i32) callconv(.C) void;

pub const P = ?fn (i32, bool, bool, i32) callconv(.C) void;

extern fn O() ?fn () callconv(.C) void;

extern fn root(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: [*c]i32, l: [*c][*c]const i32, m: [*c]?fn (i32, i32) callconv(.C) bool, n: [*c]?fn (i32, i32) callconv(.C) void, p: P) void;
