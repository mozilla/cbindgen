use std::default::Default;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::PathBuf;

use toml;

pub use bindgen::directive::*;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    Cxx,
    C,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Braces {
    SameLine,
    NextLine,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Layout {
    Horizontal,
    Vertical,
    Auto,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Config {
    /// Optional text to output at the beginning of the file
    pub header: Option<String>,
    /// Optional text to output at the end of the file
    pub trailer: Option<String>,
    /// Optional text to output at major sections to deter manual editing
    pub autogen_warning: Option<String>,
    /// Include a comment with the version of cbindgen used to generate the file
    pub include_version: bool,
    /// The style to use for braces
    pub braces: Braces,
    /// The preferred length of a line, used for auto breaking function arguments
    pub line_length: usize,
    /// The amount of spaces in a tab
    pub tab_width: usize,
    /// The language to output bindings for
    pub language: Language,
    /// The configuration options for functions
    #[serde(rename = "fn")]
    pub function: FunctionConfig,
    /// The configuration options for structs
    #[serde(rename = "struct")]
    pub structure: StructConfig,
    /// The configuration options for enums
    #[serde(rename = "enum")]
    pub enumeration: EnumConfig,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            header: None,
            trailer: None,
            autogen_warning: None,
            include_version: true,
            braces: Braces::SameLine,
            line_length: 100,
            tab_width: 2,
            language: Language::C,
            function: FunctionConfig::default(),
            structure: StructConfig::default(),
            enumeration: EnumConfig::default(),
        }
    }
}

impl Config {
    pub fn from_file(file_name: &str) -> Result<Config, String> {
        fn read(file_name: &str) -> io::Result<String> {
            let file = File::open(file_name)?;
            let mut reader = BufReader::new(&file);
            let mut contents = String::new();
            reader.read_to_string(&mut contents)?;
            Ok(contents)
        }

        let config_text = read(file_name).unwrap();

        match toml::from_str::<Config>(&config_text) {
            Ok(x) => Ok(x),
            Err(e) => Err(format!("couldn't parse config file: {}", e)),
        }
    }

    pub fn from_root_or_default(root: &str) -> Config {
        let c = PathBuf::from(root).join("cbindgen.toml");

        if c.exists() {
            Config::from_file(c.to_str().unwrap()).unwrap()
        } else {
            Config::default()
        }
    }

    pub fn from_webrender() -> Config {
        let license = r###"/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */"###;

        let autogen = r###"/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen.
 * To generate this file, clone `https://github.com/rlhunt/cbindgen` or run `cargo install cbindgen`,
 * then run `cbindgen -c wr gfx/webrender_bindings/ gfx/webrender_bindings/webrender_ffi_generated.h` */"###;

        Config {
            header: Some(String::from(license)),
            trailer: None,
            autogen_warning: Some(String::from(autogen)),
            include_version: true,
            braces: Braces::SameLine,
            line_length: 100,
            tab_width: 2,
            language: Language::Cxx,
            function: FunctionConfig {
                prefix: Some(String::from("WR_INLINE")),
                postfix: Some(String::from("WR_FUNC")),
                args: Layout::Vertical,
            },
            structure: StructConfig {
                derive_eq: true,
                derive_neq: false,
                derive_lt: false,
                derive_lte: false,
                derive_gt: false,
                derive_gte: false,
            },
            enumeration: EnumConfig {
                add_sentinel: true,
            },
        }
    }

    pub fn load(config: &str) -> Config {
        match config {
            "default" => Config::default(),
            "wr" => Config::from_webrender(),
            file => Config::from_file(file).unwrap(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct FunctionConfig {
    /// Optional text to output before each function declaration
    pub prefix: Option<String>,
    /// Optional text to output after each function declaration
    pub postfix: Option<String>,
    /// The style to layout the args
    pub args: Layout,
}

impl Default for FunctionConfig {
    fn default() -> FunctionConfig {
        FunctionConfig {
            prefix: None,
            postfix: None,
            args: Layout::Auto,
        }
    }
}

impl FunctionConfig {
    pub fn prefix(&self, directives: &DirectiveSet) -> Option<String> {
        if let Some(x) = directives.atom("function-prefix") {
            return x;
        }
        if let Some(x) = directives.atom("prefix") {
            return x;
        }
        self.prefix.clone()
    }

    pub fn postfix(&self, directives: &DirectiveSet) -> Option<String> {
        if let Some(x) = directives.atom("function-postfix") {
            return x;
        }
        if let Some(x) = directives.atom("postfix") {
            return x;
        }
        self.postfix.clone()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct StructConfig {
    /// Whether to generate a piecewise equality operator
    pub derive_eq: bool,
    /// Whether to generate a piecewise inequality operator
    pub derive_neq: bool,
    /// Whether to generate a less than operator on structs with one field
    pub derive_lt: bool,
    /// Whether to generate a less than or equal to operator on structs with one field
    pub derive_lte: bool,
    /// Whether to generate a greater than operator on structs with one field
    pub derive_gt: bool,
    /// Whether to generate a greater than or equal to operator on structs with one field
    pub derive_gte: bool,
}

impl Default for StructConfig {
    fn default() -> StructConfig {
        StructConfig {
            derive_eq: false,
            derive_neq: false,
            derive_lt: false,
            derive_lte: false,
            derive_gt: false,
            derive_gte: false,
        }
    }
}

impl StructConfig {
    pub fn derive_eq(&self, directives: &DirectiveSet) -> bool {
        if let Some(x) = directives.bool("struct-gen-op-eq") {
            return x;
        }
        if let Some(x) = directives.bool("derive-eq") {
            return x;
        }
        self.derive_eq
    }
    pub fn derive_neq(&self, directives: &DirectiveSet) -> bool {
        if let Some(x) = directives.bool("struct-gen-op-neq") {
            return x;
        }
        if let Some(x) = directives.bool("derive-neq") {
            return x;
        }
        self.derive_neq
    }
    pub fn derive_lt(&self, directives: &DirectiveSet) -> bool {
        if let Some(x) = directives.bool("struct-gen-op-lt") {
            return x;
        }
        if let Some(x) = directives.bool("derive-lt") {
            return x;
        }
        self.derive_lt
    }
    pub fn derive_lte(&self, directives: &DirectiveSet) -> bool {
        if let Some(x) = directives.bool("struct-gen-op-lte") {
            return x;
        }
        if let Some(x) = directives.bool("derive-lte") {
            return x;
        }
        self.derive_lte
    }
    pub fn derive_gt(&self, directives: &DirectiveSet) -> bool {
        if let Some(x) = directives.bool("struct-gen-op-gt") {
            return x;
        }
        if let Some(x) = directives.bool("derive-gt") {
            return x;
        }
        self.derive_gt
    }
    pub fn derive_gte(&self, directives: &DirectiveSet) -> bool {
        if let Some(x) = directives.bool("struct-gen-op-gte") {
            return x;
        }
        if let Some(x) = directives.bool("derive-gte") {
            return x;
        }
        self.derive_gte
    }
}

#[derive( Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct EnumConfig {
    /// Whether to add a `Sentinel` value at the end of every enum
    /// This is useful in Gecko for IPC serialization
    pub add_sentinel: bool,
}

impl Default for EnumConfig {
    fn default() -> EnumConfig {
        EnumConfig {
            add_sentinel: false,
        }
    }
}

impl EnumConfig {
    pub fn add_sentinel(&self, directives: &DirectiveSet) -> bool {
        if let Some(x) = directives.bool("enum-add-sentinel") {
            return x;
        }
        if let Some(x) = directives.bool("add-sentinel") {
            return x;
        }
        self.add_sentinel
    }
}
