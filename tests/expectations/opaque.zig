const std = @import("std");

const HashMap_i32__i32__BuildHasherDefault_DefaultHasher = opaque {};

const Result_Foo = opaque {};

/// Fast hash map used internally.
pub const FastHashMap_i32__i32 = HashMap_i32__i32__BuildHasherDefault_DefaultHasher;

pub const Foo = FastHashMap_i32__i32;

pub const Bar = Result_Foo;

pub extern fn root(a: ?*Foo, b: ?*Bar) anyopaque;
