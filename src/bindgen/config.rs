use std::fs::File;

use serde_json;

pub use bindgen::directive::*;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Deserialize)]
pub struct FileConfig {
    /// Optional text to output at the beginning of the file
    pub header: Option<String>,
    /// Optional text to output at the end of the file
    pub trailer: Option<String>,
    /// Optional text to output at major sections to deter manual editing
    pub autogen_warning: Option<String>,
    /// Include a comment with the version of cbindgen used to generate the file
    pub include_version: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ItemConfig {
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

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub global: FileConfig,
    pub per_item: ItemConfig,
}

impl FileConfig {
    pub fn default() -> FileConfig {
        FileConfig {
            header: None,
            trailer: None,
            autogen_warning: None,
            include_version: false,
        }
    }
}

impl ItemConfig {
    pub fn default() -> ItemConfig {
        ItemConfig {
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

    pub fn function_prefix(&self, directives: &DirectiveSet) -> Option<String> {
        match directives.atom("function-prefix") {
            Some(x) => x,
            None => self.function_prefix.clone(),
        }
    }
    pub fn function_postfix(&self, directives: &DirectiveSet) -> Option<String> {
        match directives.atom("function-postfix") {
            Some(x) => x,
            None => self.function_postfix.clone(),
        }
    }

    pub fn enum_add_sentinel(&self, directives: &DirectiveSet) -> bool {
        match directives.bool("enum-add-sentinel") {
            Some(x) => x,
            None => self.enum_add_sentinel,
        }
    }

    pub fn struct_gen_op_eq(&self, directives: &DirectiveSet) -> bool {
        match directives.bool("struct-gen-op-eq") {
            Some(x) => x,
            None => self.struct_gen_op_eq,
        }
    }
    pub fn struct_gen_op_neq(&self, directives: &DirectiveSet) -> bool {
        match directives.bool("struct-gen-op-neq") {
            Some(x) => x,
            None => self.struct_gen_op_neq,
        }
    }
    pub fn struct_gen_op_lt(&self, directives: &DirectiveSet) -> bool {
        match directives.bool("struct-gen-op-lt") {
            Some(x) => x,
            None => self.struct_gen_op_lt,
        }
    }
    pub fn struct_gen_op_lte(&self, directives: &DirectiveSet) -> bool {
        match directives.bool("struct-gen-op-lte") {
            Some(x) => x,
            None => self.struct_gen_op_lte,
        }
    }
    pub fn struct_gen_op_gt(&self, directives: &DirectiveSet) -> bool {
        match directives.bool("struct-gen-op-gt") {
            Some(x) => x,
            None => self.struct_gen_op_gt,
        }
    }
    pub fn struct_gen_op_gte(&self, directives: &DirectiveSet) -> bool {
        match directives.bool("struct-gen-op-gte") {
            Some(x) => x,
            None => self.struct_gen_op_gte,
        }
    }
}

impl Config {

    pub fn from_default() -> Config {
        Config {
            global: FileConfig::default(),
            per_item: ItemConfig::default(),
        }
    }

    pub fn from_file(file: &str) -> Config {
        serde_json::from_reader(&File::open(file).unwrap()).unwrap()
    }

    pub fn from_webrender() -> Config {
        let license = r###"/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */"###;

        let autogen = r###"/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen.
 * To generate this file, clone `https://github.com/rlhunt/cbindgen` or run `cargo install cbindgen`,
 * then run `cbindgen -c wr gfx/webrender_bindings/ gfx/webrender_bindings/webrender_ffi_generated.h` */"###;

        Config {
            global: FileConfig {
                header: Some(String::from(license)),
                trailer: None,
                autogen_warning: Some(String::from(autogen)),
                include_version: true,
            },
            per_item: ItemConfig {
                function_prefix: Some(String::from("WR_INLINE")),
                function_postfix: Some(String::from("WR_FUNC")),
                enum_add_sentinel: true,
                struct_gen_op_eq: true,
                struct_gen_op_neq: false,
                struct_gen_op_lt: false,
                struct_gen_op_lte: false,
                struct_gen_op_gt: false,
                struct_gen_op_gte: false,
            },
        }
    }

    pub fn load(config: &str) -> Config {
        match config {
            "default" => Config::from_default(),
            "wr" => Config::from_webrender(),
            file => Config::from_file(file),
        }
    }
}
