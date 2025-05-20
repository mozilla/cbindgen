module cbindgen;

@nogc nothrow @safe:

extern(C) {

/// The root of all evil.
///
/// But at least it contains some more documentation as someone would expect
/// from a simple test case like this. Though, this shouldn't appear in the
/// output.
void root();

/// A little above the root, and a lot more visible, with a run-on sentence
/// to test going over the first line.
///
/// Still not here, though.
void trunk();

}  // extern(C)
