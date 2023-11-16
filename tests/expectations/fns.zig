const std = @import("std");

pub const Fns = extern struct {
   _noArgs: ?fn() anyopaque,
   _anonymousArg: ?fn() anyopaque,
   _returnsNumber: ?fn() i32,
   _namedArgs: ?fn(first: i32, snd: i16) i8,
   _namedArgsWildcards: ?fn(_: i32, named: i16, _1: i64) i8,
};

extern fn root(_fns: Fns) anyopaque;

extern fn no_return() anyopaque;
