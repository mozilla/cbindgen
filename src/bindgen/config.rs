use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::default::Default;

use toml;

pub use bindgen::directive::*;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Config {
    pub file: FileConfig,
    #[serde(rename = "fn")]
    pub function: FunctionConfig,
    #[serde(rename = "struct")]
    pub structure: StructConfig,
    #[serde(rename = "enum")]
    pub enumeration: EnumConfig,
}

impl Config {
    pub fn from_file(file_name: &str) -> Config {
        fn read(file_name: &str) -> io::Result<String> {
            let file = File::open(file_name)?;
            let mut reader = BufReader::new(&file);
            let mut contents = String::new();
            reader.read_to_string(&mut contents)?;
            Ok(contents)
        }

        let config_text = read(file_name).expect("couldn't open config file");
        toml::from_str(&config_text).expect("couldn't parse config file")
    }

    pub fn from_webrender() -> Config {
        let license = r###"/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */"###;

        let autogen = r###"/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen.
 * To generate this file, clone `https://github.com/rlhunt/cbindgen` or run `cargo install cbindgen`,
 * then run `cbindgen -c wr gfx/webrender_bindings/ gfx/webrender_bindings/webrender_ffi_generated.h` */"###;

        Config {
            file: FileConfig {
                header: Some(String::from(license)),
                trailer: None,
                autogen_warning: Some(String::from(autogen)),
                include_version: Some(true),
            },
            function: FunctionConfig {
                prefix: Some(String::from("WR_INLINE")),
                postfix: Some(String::from("WR_FUNC")),
            },
            structure: StructConfig {
                derive_op_eq: Some(true),
                derive_op_neq: Some(false),
                derive_op_lt: Some(false),
                derive_op_lte: Some(false),
                derive_op_gt: Some(false),
                derive_op_gte: Some(false),
            },
            enumeration: EnumConfig {
                add_sentinel: Some(true),
            },
        }
    }

    pub fn load(config: &str) -> Config {
        match config {
            "default" => Config::default(),
            "wr" => Config::from_webrender(),
            file => Config::from_file(file),
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct FileConfig {
    /// Optional text to output at the beginning of the file
    pub header: Option<String>,
    /// Optional text to output at the end of the file
    pub trailer: Option<String>,
    /// Optional text to output at major sections to deter manual editing
    pub autogen_warning: Option<String>,
    /// Include a comment with the version of cbindgen used to generate the file
    pub include_version: Option<bool>,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct FunctionConfig {
    /// Optional text to output before each function declaration
    pub prefix: Option<String>,
    /// Optional text to output after each function declaration
    pub postfix: Option<String>,
}

impl FunctionConfig {
    pub fn prefix(&self, directives: &DirectiveSet) -> Option<String> {
        match directives.atom("function-prefix") {
            Some(x) => x,
            None => self.prefix.clone(),
        }
    }

    pub fn postfix(&self, directives: &DirectiveSet) -> Option<String> {
        match directives.atom("function-postfix") {
            Some(x) => x,
            None => self.postfix.clone(),
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct StructConfig {
    /// Whether to generate a piecewise equality operator
    pub derive_op_eq: Option<bool>,
    /// Whether to generate a piecewise inequality operator
    pub derive_op_neq: Option<bool>,
    /// Whether to generate a less than operator on structs with one field
    pub derive_op_lt: Option<bool>,
    /// Whether to generate a less than or equal to operator on structs with one field
    pub derive_op_lte: Option<bool>,
    /// Whether to generate a greater than operator on structs with one field
    pub derive_op_gt: Option<bool>,
    /// Whether to generate a greater than or equal to operator on structs with one field
    pub derive_op_gte: Option<bool>,
}

impl StructConfig {
    pub fn derive_op_eq(&self, directives: &DirectiveSet) -> bool {
        match directives.bool("struct-gen-op-eq") {
            Some(x) => x,
            None => self.derive_op_eq.unwrap_or(false),
        }
    }
    pub fn derive_op_neq(&self, directives: &DirectiveSet) -> bool {
        match directives.bool("struct-gen-op-neq") {
            Some(x) => x,
            None => self.derive_op_neq.unwrap_or(false),
        }
    }
    pub fn derive_op_lt(&self, directives: &DirectiveSet) -> bool {
        match directives.bool("struct-gen-op-lt") {
            Some(x) => x,
            None => self.derive_op_lt.unwrap_or(false),
        }
    }
    pub fn derive_op_lte(&self, directives: &DirectiveSet) -> bool {
        match directives.bool("struct-gen-op-lte") {
            Some(x) => x,
            None => self.derive_op_lte.unwrap_or(false),
        }
    }
    pub fn derive_op_gt(&self, directives: &DirectiveSet) -> bool {
        match directives.bool("struct-gen-op-gt") {
            Some(x) => x,
            None => self.derive_op_gt.unwrap_or(false),
        }
    }
    pub fn derive_op_gte(&self, directives: &DirectiveSet) -> bool {
        match directives.bool("struct-gen-op-gte") {
            Some(x) => x,
            None => self.derive_op_gte.unwrap_or(false),
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct EnumConfig {
    /// Whether to add a `Sentinel` value at the end of every enum
    /// This is useful in Gecko for IPC serialization
    pub add_sentinel: Option<bool>,
}

impl EnumConfig {
    pub fn add_sentinel(&self, directives: &DirectiveSet) -> bool {
        match directives.bool("enum-add-sentinel") {
            Some(x) => x,
            None => self.add_sentinel.unwrap_or(false),
        }
    }
}
