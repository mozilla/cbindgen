This project can be used to generate C bindings for rust code. In particular it will be used for generating a C API for webrender. It is not yet fully complete.

Steps to use:
- check out the `bindgen` branch of https://github.com/staktrace/gecko-dev (this has a few patches that make binding generation easier)
- cargo run path/to/bindgen-branch/gfx/webrender\_bindings > path/to/bindgen-branch/gfx/webrender\_bindings/webrender\_ffi\_autogen.h
- Try to compile the `bindgen` branch. Deal with the errors by fixing bindgen

Future work:
- Make bindgen generate even more things, so that webrender\_ffi.h is minimal.
