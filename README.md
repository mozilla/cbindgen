# `cbindgen`

This project can be used to generate C bindings for rust code. It is currently being developed to support creating bindings for [WebRender](https://github.com/servo/webrender/).

## Use

`cbindgen path_to_crate > autogen.h`

## How it works

1. All the structs, enums, type aliases, and functions that are representable in C are gathered
2. A dependency graph is built using the extern "C" functions as roots
  * This removes unneeded types from the bindings and sorts the structs that depend on each other
3. Some code generation is done to specialize generics that are specified as type aliases
3. The items are printed in dependency order in C syntax

## Future work

1. Add code for customizing bindings and removing code specific to WebRender
2. Improve the CLI
3. Better logging
4. Sort the output types better
5. Better support for types with fully specified names
6. Add a validation step
7. ...
