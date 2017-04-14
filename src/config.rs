pub struct Config {
    /// Optional text to output at the beginning of the file
    pub file_header: Option<String>,
    /// Optional text to output at the end of the file
    pub file_trailer: Option<String>,
    /// Optional text to output at major sections to deter manual editing
    pub file_autogen_warning: Option<String>,
    /// Optional text to output before each function declaration
    pub function_prefix: Option<String>,
    /// Optional text to output after each function declaration
    pub function_postfix: Option<String>,
    /// Whether to add a `Sentinel` value at the end of every enum
    /// This is useful in Gecko for IPC serialization
    pub enum_add_sentinel: bool,
    /// Whether to generate a piecewise equality operator
    pub struct_gen_op_eq: bool,
    /// Whether to generate a piecewise inequality operator
    pub struct_gen_op_neq: bool,
    /// Whether to generate a less than operator on structs with one field
    pub struct_gen_op_lt: bool,
    /// Whether to generate a less than or equal to operator on structs with one field
    pub struct_gen_op_lte: bool,
    /// Whether to generate a greater than operator on structs with one field
    pub struct_gen_op_gt: bool,
    /// Whether to generate a greater than or equal to operator on structs with one field
    pub struct_gen_op_gte: bool,
}

impl Config {
    pub fn default() -> Config {
        Config {
            file_header: None,
            file_trailer: None,
            file_autogen_warning: None,
            function_prefix: None,
            function_postfix: None,
            enum_add_sentinel: false,
            struct_gen_op_eq: false,
            struct_gen_op_neq: false,
            struct_gen_op_lt: false,
            struct_gen_op_lte: false,
            struct_gen_op_gt: false,
            struct_gen_op_gte: false,
        }
    }

    pub fn gecko_webrender() -> Config {
        let license = r###"/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */"###;

        let autogen = r###"/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen.
 * To generate this file, clone `https://github.com/rlhunt/cbindgen` or run `cargo install cbindgen`,
 * then run `cbindgen -c wr gfx/webrender_bindings/ gfx/webrender_bindings/webrender_ffi_generated.h` */"###;

        Config {
            file_header: Some(String::from(license)),
            file_trailer: None,
            file_autogen_warning: Some(String::from(autogen)),
            function_prefix: Some(String::from("WR_INLINE")),
            function_postfix: Some(String::from("WR_FUNC")),
            enum_add_sentinel: true,
            struct_gen_op_eq: true,
            struct_gen_op_neq: false,
            struct_gen_op_lt: false,
            struct_gen_op_lte: false,
            struct_gen_op_gt: false,
            struct_gen_op_gte: false,
        }
    }

    pub fn load(config: &str) -> Option<Config> {
        match config {
            "default" => Some(Config::default()),
            "wr" => Some(Config::gecko_webrender()),
            _ => None,
        }
    }
}
