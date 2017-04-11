This project can be used to generate C bindings for rust code. In particular it will be used for generating a C API for webrender.

Steps to use:
- cargo run path/to/mozilla/gfx/webrender\_bindings > path/to/mozilla/gfx/webrender\_bindings/webrender\_ffi\_generated.h
- Compile and test your mozilla build. Deal with any errors by fixing the wr-binding generator or other things in the mozilla tree.

The first step above will spew a bunch of output to stderr - you can ignore it unless the run fails. The way wr-binding works
is by processing the .rs files and looking for function signatures with the `no_mangle` attribute on them. These functions will
have C signatures generated. Furthermore, the generator looks for structs with `repr(C)` and enums with `repr(u32)`, and if they
are used by the `no_mangle` functions, generates bindings for those as well. If you try to use a struct that is not `repr(C)` or
an enum that is not `repr(u32)` the generator will just ignore it and you will probably get a build failure in your mozilla
tree.

Future work:
- Make wr-binding generate even more things, so that webrender\_ffi.h is minimal.
