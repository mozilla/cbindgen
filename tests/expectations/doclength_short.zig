const std = @import("std");

// The root of all evil.
//
// But at least it contains some more documentation as someone would expect
// from a simple test case like this. Though, this shouldn't appear in the
// output.
extern fn root() anyopaque;

// A little above the root, and a lot more visible, with a run-on sentence
// to test going over the first line.
//
// Still not here, though.
extern fn trunk() anyopaque;
