# `cbindgen` &emsp; [![Build Status]][travis] [![Latest Version]][crates.io]

[Build Status]: https://api.travis-ci.org/rlhunt/cbindgen.svg?branch=master
[travis]: https://travis-ci.org/rlhunt/cbindgen
[Latest Version]: https://img.shields.io/crates/v/cbindgen.svg
[crates.io]: https://crates.io/crates/cbindgen

This project can be used to generate C bindings for Rust code. It is currently being developed to support creating bindings for [WebRender](https://github.com/servo/webrender/), but has been designed to support any project.

## Features

  * Builds bindings for a crate, its mods, its dependent crates, and their mods
  * Only the necessary types for exposed functions are given bindings
  * Can specify annotations for controlling some aspects of binding
  * Generic structs can be exposed using `type IntFoo = Foo<i32>;`
  * Customizable formatting, can be used in C or C++ projects

## Use

### Command line

`cbindgen crate/ -o crate/bindings.h`

See `cbindgen --help` for more options.

### `build.rs`

`cbindgen` can also be used in build scripts. How this fits into compiling the native code depends on your project.

Here's an example build.rs script:
```rust
extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::generate(crate_dir)
      .unwrap()
      .write_to_file("bindings.h");
}

```

## Configuration

There are some options that can be used to configure the binding generation. They can be specified by creating a `cbindgen.toml` with the options in the binding crate root. Alternatively, build scripts can specify them using `cbindgen::generate_with_config`.

Some useful options:

1. `header` - optional text to output at the beginning of the file
2. `trailer` - optional text to output at the end of the file
3. `include_guard` - optional name to use for an include guard
4. `autogen_warning` - optional text to output at major sections to deter manual editing
5. `include_version` - whether to include a comment with the version of cbindgen used to generate the file
6. `braces` - the style to use for braces (can be either SameLine or NextLine)
7. `line_length` - the preferred length of a line, used when auto breaking function arguments
8. `tab_width` - the amount of spaces in an indentation
9. `language` - the language to generate bindings in (can be either C++ or C)
10. `parse_deps` - whether to parse dependent crates
11. `include` - an optional whitelist to use when parsing dependent crates
12. `exclude` - an optional blacklist to use when parsing dependent crates

A full listing of options can be found in `src/bindgen/config.rs`

## Examples

See `compile-tests/` for some examples of rust source that can be handled.

## How it works

1. All the structs, enums, type aliases, and functions that are representable in C are gathered
2. A dependency graph is built using the extern "C" functions as roots
    * This removes unneeded types from the bindings and sorts the structs that depend on each other
3. Some code generation is done to specialize generics that are specified as type aliases
3. The items are printed in dependency order in C syntax

## Future work

1. Better support for types with fully specified names
2. Support for generating a FFI interface for a Struct+Impl
3. ...
