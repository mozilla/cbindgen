This project can be used to generate C bindings for rust code. In particular it will be used for generating a C API for webrender. It is not yet fully complete.

Steps to use:
- check out the `bindgen` branch of https://github.com/staktrace/gecko-dev (this has a few patches that make binding generation easier)
- cargo run path/to/bindgen-branch/gfx/webrender\_bindings > webrender\_ffi\_autogen.h
- Put the generated file into gfx/webrender\_bindings and #include it from webrender\_ffi.h
- A bunch of the stuff from webrender\_ffi.h can now be removed
