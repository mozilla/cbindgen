/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::default::Default;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::Path;
use std::str::FromStr;

use toml;

pub use bindgen::annotation::*;
pub use bindgen::rename::*;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// A language type to generate bindings for.
#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    Cxx,
    C,
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Language, Self::Err> {
        match s {
            "cxx" => Ok(Language::Cxx),
            "Cxx" => Ok(Language::Cxx),
            "CXX" => Ok(Language::Cxx),
            "cpp" => Ok(Language::Cxx),
            "Cpp" => Ok(Language::Cxx),
            "CPP" => Ok(Language::Cxx),
            "c++" => Ok(Language::Cxx),
            "C++" => Ok(Language::Cxx),
            "c" => Ok(Language::C),
            "C" => Ok(Language::C),
            _ => Err(format!("unrecognized Language: '{}'", s)),
        }
    }
}

deserialize_enum_str!(Language);

/// A style of braces to use for generating code.
#[derive(Debug, Clone, PartialEq)]
pub enum Braces {
    SameLine,
    NextLine,
}

impl FromStr for Braces {
    type Err = String;

    fn from_str(s: &str) -> Result<Braces, Self::Err> {
        match s {
            "SameLine" => Ok(Braces::SameLine),
            "same_line" => Ok(Braces::SameLine),
            "NextLine" => Ok(Braces::NextLine),
            "next_line" => Ok(Braces::NextLine),
            _ => Err(format!("unrecognized Braces: '{}'", s)),
        }
    }
}

deserialize_enum_str!(Braces);

/// A type of layout to use when generating long lines of code.
#[derive(Debug, Clone, PartialEq)]
pub enum Layout {
    Horizontal,
    Vertical,
    Auto,
}

impl FromStr for Layout {
    type Err = String;

    fn from_str(s: &str) -> Result<Layout, Self::Err> {
        match s {
            "Horizontal" => Ok(Layout::Horizontal),
            "horizontal" => Ok(Layout::Horizontal),
            "Vertical" => Ok(Layout::Vertical),
            "vertical" => Ok(Layout::Vertical),
            "Auto" => Ok(Layout::Auto),
            "auto" => Ok(Layout::Auto),
            _ => Err(format!("unrecognized Layout: '{}'", s)),
        }
    }
}

deserialize_enum_str!(Layout);

/// Settings to apply to generated functions.
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
    /// The rename rule to apply to function args
    pub rename_args: Option<RenameRule>,
}

impl Default for FunctionConfig {
    fn default() -> FunctionConfig {
        FunctionConfig {
            prefix: None,
            postfix: None,
            args: Layout::Auto,
            rename_args: None,
        }
    }
}

impl FunctionConfig {
    pub fn prefix(&self, annotations: &AnnotationSet) -> Option<String> {
        if let Some(x) = annotations.atom("prefix") {
            return x;
        }
        self.prefix.clone()
    }

    pub fn postfix(&self, annotations: &AnnotationSet) -> Option<String> {
        if let Some(x) = annotations.atom("postfix") {
            return x;
        }
        self.postfix.clone()
    }
}

/// Settings to apply to generated structs.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct StructConfig {
    /// The rename rule to apply to the name of struct fields
    pub rename_fields: Option<RenameRule>,
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
            rename_fields: None,
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
    pub fn derive_eq(&self, annotations: &AnnotationSet) -> bool {
        if let Some(x) = annotations.bool("derive-eq") {
            return x;
        }
        self.derive_eq
    }
    pub fn derive_neq(&self, annotations: &AnnotationSet) -> bool {
        if let Some(x) = annotations.bool("derive-neq") {
            return x;
        }
        self.derive_neq
    }
    pub fn derive_lt(&self, annotations: &AnnotationSet) -> bool {
        if let Some(x) = annotations.bool("derive-lt") {
            return x;
        }
        self.derive_lt
    }
    pub fn derive_lte(&self, annotations: &AnnotationSet) -> bool {
        if let Some(x) = annotations.bool("derive-lte") {
            return x;
        }
        self.derive_lte
    }
    pub fn derive_gt(&self, annotations: &AnnotationSet) -> bool {
        if let Some(x) = annotations.bool("derive-gt") {
            return x;
        }
        self.derive_gt
    }
    pub fn derive_gte(&self, annotations: &AnnotationSet) -> bool {
        if let Some(x) = annotations.bool("derive-gte") {
            return x;
        }
        self.derive_gte
    }
}

/// Settings to apply to generated enums.
#[derive( Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct EnumConfig {
    /// The rename rule to apply to the name of enum variants
    pub rename_variants: Option<RenameRule>,
    /// Whether to add a `Sentinel` value at the end of every enum
    /// This is useful in Gecko for IPC serialization
    pub add_sentinel: bool,
}

impl Default for EnumConfig {
    fn default() -> EnumConfig {
        EnumConfig {
            rename_variants: None,
            add_sentinel: false,
        }
    }
}

impl EnumConfig {
    pub fn add_sentinel(&self, annotations: &AnnotationSet) -> bool {
        if let Some(x) = annotations.bool("add-sentinel") {
            return x;
        }
        self.add_sentinel
    }
}

/// Settings to apply when parsing.
#[derive( Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct ParseConfig {
    /// Whether to parse dependencies when generating bindings. When this is true,
    /// each dependent crate is found using a combination of `cargo metadata` and
    /// `Cargo.lock`. To further control this behavior, crates can be whitelisted or
    /// blacklisted using `include` and `exclude` respectively. Additionally in cases
    /// where crates have types to expose in bindings hidden in macros, a crate can
    /// be marked in `expand` and `cargo expand` will be used to expand the macros
    /// before parsing. A crate marked in `expand` doesn't need to be added to any
    /// whitelist.
    pub parse_deps: bool,
    /// An optional whitelist of names of crates to parse
    pub include: Option<Vec<String>>,
    /// The names of crates to not parse
    pub exclude: Vec<String>,
    /// The names of crates to parse with `rustc --pretty=expanded`
    pub expand: Vec<String>,
}

impl Default for ParseConfig {
    fn default() -> ParseConfig {
        ParseConfig {
            parse_deps: false,
            include: None,
            exclude: Vec::new(),
            expand: Vec::new(),
        }
    }
}

/// A collection of settings to customize the generated bindings.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Config {
    /// Optional text to output at the beginning of the file
    pub header: Option<String>,
    /// Optional text to output at the end of the file
    pub trailer: Option<String>,
    /// Optional name to use for an include guard
    pub include_guard: Option<String>,
    /// Optional text to output at major sections to deter manual editing
    pub autogen_warning: Option<String>,
    /// Include a comment with the version of cbindgen used to generate the file
    pub include_version: bool,
    /// An optional name for the root namespace. Only applicable when language="C++"
    pub namespace: Option<String>,
    /// The style to use for braces
    pub braces: Braces,
    /// The preferred length of a line, used for auto breaking function arguments
    pub line_length: usize,
    /// The amount of spaces in a tab
    pub tab_width: usize,
    /// The language to output bindings for
    pub language: Language,
    /// The configuration options for parsing
    pub parse: ParseConfig,
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
            include_guard: None,
            autogen_warning: None,
            include_version: true,
            namespace: None,
            braces: Braces::SameLine,
            line_length: 100,
            tab_width: 2,
            language: Language::Cxx,
            parse: ParseConfig::default(),
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

    pub fn from_root_or_default(root: &Path) -> Config {
        let c = root.join("cbindgen.toml");

        if c.exists() {
            Config::from_file(c.to_str().unwrap()).unwrap()
        } else {
            Config::default()
        }
    }
}
