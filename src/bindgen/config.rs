/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;
use std::default::Default;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::Path;
use std::str::FromStr;

use serde::de::value::{MapAccessDeserializer, SeqAccessDeserializer};
use serde::de::{Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};

use toml;

use bindgen::ir::annotation::AnnotationSet;
pub use bindgen::rename::RenameRule;

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
            _ => Err(format!("Unrecognized Language: '{}'.", s)),
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
            _ => Err(format!("Unrecognized Braces: '{}'.", s)),
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
            _ => Err(format!("Unrecognized Layout: '{}'.", s)),
        }
    }
}

deserialize_enum_str!(Layout);

/// A style of Style to use when generating structs and enums.
#[derive(Debug, Clone, PartialEq)]
pub enum Style {
    Both,
    Tag,
    Type,
}

impl Style {
    pub fn generate_tag(&self) -> bool {
        match self {
            &Style::Both => true,
            &Style::Tag => true,
            &Style::Type => false,
        }
    }

    pub fn generate_typedef(&self) -> bool {
        match self {
            &Style::Both => true,
            &Style::Tag => false,
            &Style::Type => true,
        }
    }
}

impl FromStr for Style {
    type Err = String;

    fn from_str(s: &str) -> Result<Style, Self::Err> {
        match s {
            "Both" => Ok(Style::Both),
            "both" => Ok(Style::Both),
            "Tag" => Ok(Style::Tag),
            "tag" => Ok(Style::Tag),
            "Type" => Ok(Style::Type),
            "type" => Ok(Style::Type),
            _ => Err(format!("Unrecognized Style: '{}'.", s)),
        }
    }
}

deserialize_enum_str!(Style);

/// Different item types that we can generate and filter.
#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    Constants,
    Globals,
    Enums,
    Structs,
    Unions,
    Typedefs,
    OpaqueItems,
    Functions,
}

impl FromStr for ItemType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::ItemType::*;
        Ok(match &*s.to_lowercase() {
            "constants" => Constants,
            "globals" => Globals,
            "enums" => Enums,
            "structs" => Structs,
            "unions" => Unions,
            "typedefs" => Typedefs,
            "opaque" => OpaqueItems,
            "functions" => Functions,
            _ => return Err(format!("Unrecognized Style: '{}'.", s)),
        })
    }
}

deserialize_enum_str!(ItemType);

/// Settings to apply when exporting items.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct ExportConfig {
    /// A list of additional items not used by exported functions to include in
    /// the generated bindings
    pub include: Vec<String>,
    /// A list of items to not include in the generated bindings
    pub exclude: Vec<String>,
    /// Table of name conversions to apply to item names
    pub rename: HashMap<String, String>,
    /// A prefix to add before the name of every item
    pub prefix: Option<String>,
    /// Types of items to generate.
    pub item_types: Vec<ItemType>,
}

impl Default for ExportConfig {
    fn default() -> ExportConfig {
        ExportConfig {
            include: Vec::new(),
            exclude: Vec::new(),
            rename: HashMap::new(),
            prefix: None,
            item_types: Vec::new(),
        }
    }
}

impl ExportConfig {
    pub(crate) fn should_generate(&self, item_type: ItemType) -> bool {
        self.item_types.is_empty() || self.item_types.contains(&item_type)
    }

    pub(crate) fn rename(&self, item_name: &mut String) {
        if let Some(name) = self.rename.get(item_name) {
            *item_name = name.clone();
        }
        if let Some(ref prefix) = self.prefix {
            item_name.insert_str(0, &prefix);
        }
    }
}

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
    pub(crate) fn prefix(&self, annotations: &AnnotationSet) -> Option<String> {
        if let Some(x) = annotations.atom("prefix") {
            return x;
        }
        self.prefix.clone()
    }

    pub(crate) fn postfix(&self, annotations: &AnnotationSet) -> Option<String> {
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
    /// Whether to generate a constructor for the struct (which takes
    /// arguments to initialize all the members)
    pub derive_constructor: bool,
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
            derive_constructor: false,
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
    pub(crate) fn derive_constructor(&self, annotations: &AnnotationSet) -> bool {
        if let Some(x) = annotations.bool("derive-constructor") {
            return x;
        }
        self.derive_constructor
    }
    pub(crate) fn derive_eq(&self, annotations: &AnnotationSet) -> bool {
        if let Some(x) = annotations.bool("derive-eq") {
            return x;
        }
        self.derive_eq
    }
    pub(crate) fn derive_neq(&self, annotations: &AnnotationSet) -> bool {
        if let Some(x) = annotations.bool("derive-neq") {
            return x;
        }
        self.derive_neq
    }
    pub(crate) fn derive_lt(&self, annotations: &AnnotationSet) -> bool {
        if let Some(x) = annotations.bool("derive-lt") {
            return x;
        }
        self.derive_lt
    }
    pub(crate) fn derive_lte(&self, annotations: &AnnotationSet) -> bool {
        if let Some(x) = annotations.bool("derive-lte") {
            return x;
        }
        self.derive_lte
    }
    pub(crate) fn derive_gt(&self, annotations: &AnnotationSet) -> bool {
        if let Some(x) = annotations.bool("derive-gt") {
            return x;
        }
        self.derive_gt
    }
    pub(crate) fn derive_gte(&self, annotations: &AnnotationSet) -> bool {
        if let Some(x) = annotations.bool("derive-gte") {
            return x;
        }
        self.derive_gte
    }
}

/// Settings to apply to generated enums.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct EnumConfig {
    /// The rename rule to apply to the name of enum variants
    pub rename_variants: Option<RenameRule>,
    /// Whether to add a `Sentinel` value at the end of every enum
    /// This is useful in Gecko for IPC serialization
    pub add_sentinel: bool,
    /// Whether the enum variants should be prefixed with the enum name
    pub prefix_with_name: bool,
    /// Whether to generate static `::X(..)` constructors and `IsX()`
    /// methods for tagged enums.
    pub derive_helper_methods: bool,
}

impl Default for EnumConfig {
    fn default() -> EnumConfig {
        EnumConfig {
            rename_variants: None,
            add_sentinel: false,
            prefix_with_name: false,
            derive_helper_methods: false,
        }
    }
}

impl EnumConfig {
    pub(crate) fn add_sentinel(&self, annotations: &AnnotationSet) -> bool {
        if let Some(x) = annotations.bool("add-sentinel") {
            return x;
        }
        self.add_sentinel
    }
    pub(crate) fn derive_helper_methods(&self, annotations: &AnnotationSet) -> bool {
        if let Some(x) = annotations.bool("derive-helper-methods") {
            return x;
        }
        self.derive_helper_methods
    }
}

/// Settings to apply to generated constants.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct ConstantConfig {
    /// Whether a generated constant can be a static const in C++ mode.
    pub allow_static_const: bool,
}

impl Default for ConstantConfig {
    fn default() -> ConstantConfig {
        ConstantConfig {
            allow_static_const: true,
        }
    }
}

/// Settings to apply when running `rustc --pretty=expanded`
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct ParseExpandConfig {
    /// The names of crates to parse with `rustc --pretty=expanded`
    pub crates: Vec<String>,
    /// Whether to enable all the features when expanding.
    pub all_features: bool,
    /// Whether to use the default feature set when expanding.
    pub default_features: bool,
    /// List of features to use when expanding. Combines with `default_features` like in
    /// `Cargo.toml`.
    pub features: Option<Vec<String>>,
}

impl Default for ParseExpandConfig {
    fn default() -> ParseExpandConfig {
        ParseExpandConfig {
            crates: Vec::new(),
            all_features: false,
            default_features: true,
            features: None,
        }
    }
}

// Backwards-compatibility deserializer for ParseExpandConfig. This allows accepting both the
// simple `expand = ["crate"]` and the more complex `expand = {"crates": ["crate"],
// "default_features": false}` format for the `expand` key.
//
// Note that one (major) difference between the two forms is that, for backwards-compatibility
// reasons, the `expand = ["crate"]` form will enable the `--all-features` flag by default while
// the `expand = {"crates": ["crate"]}` form will use the default feature set by default.
fn retrocomp_parse_expand_config_deserialize<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<ParseExpandConfig, D::Error> {
    struct ParseExpandVisitor;

    impl<'de> Visitor<'de> for ParseExpandVisitor {
        type Value = ParseExpandConfig;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map or sequence of string")
        }

        fn visit_seq<A: SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
            let crates =
                <Vec<String> as Deserialize>::deserialize(SeqAccessDeserializer::new(seq))?;
            Ok(ParseExpandConfig {
                crates,
                all_features: true,
                default_features: true,
                features: None,
            })
        }

        fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
            <ParseExpandConfig as Deserialize>::deserialize(MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(ParseExpandVisitor)
}

/// Settings to apply when parsing.
#[derive(Debug, Clone, Deserialize)]
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
    /// The configuration options for `rustc --pretty=expanded`
    #[serde(deserialize_with = "retrocomp_parse_expand_config_deserialize")]
    pub expand: ParseExpandConfig,
    /// Whether to use a new temporary target directory when running `rustc --pretty=expanded`.
    /// This may be required for some build processes.
    pub clean: bool,
}

impl Default for ParseConfig {
    fn default() -> ParseConfig {
        ParseConfig {
            parse_deps: false,
            include: None,
            exclude: Vec::new(),
            expand: ParseExpandConfig::default(),
            clean: false,
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
    /// A list of additional includes to put at the beginning of the generated header
    pub includes: Vec<String>,
    /// A list of additional system includes to put at the beginning of the generated header
    pub sys_includes: Vec<String>,
    /// Optional text to output at the end of the file
    pub trailer: Option<String>,
    /// Optional name to use for an include guard
    pub include_guard: Option<String>,
    /// Generates no includes at all. Overrides all other include options
    ///
    /// This option is useful when using cbindgen with tools such as python's cffi which
    /// doesn't understand include directives
    pub no_includes: bool,
    /// Optional text to output at major sections to deter manual editing
    pub autogen_warning: Option<String>,
    /// Include a comment with the version of cbindgen used to generate the file
    pub include_version: bool,
    /// An optional name for the root namespace. Only applicable when language="C++"
    pub namespace: Option<String>,
    /// An optional list of namespaces. Only applicable when language="C++"
    pub namespaces: Option<Vec<String>>,
    /// The style to use for braces
    pub braces: Braces,
    /// The preferred length of a line, used for auto breaking function arguments
    pub line_length: usize,
    /// The amount of spaces in a tab
    pub tab_width: usize,
    /// The language to output bindings for
    pub language: Language,
    /// The style to declare structs, enums and unions in for C
    pub style: Style,
    /// The configuration options for parsing
    pub parse: ParseConfig,
    /// The configuration options for exporting
    pub export: ExportConfig,
    /// The configuration options for functions
    #[serde(rename = "fn")]
    pub function: FunctionConfig,
    /// The configuration options for structs
    #[serde(rename = "struct")]
    pub structure: StructConfig,
    /// The configuration options for enums
    #[serde(rename = "enum")]
    pub enumeration: EnumConfig,
    /// The configuration options for constants
    #[serde(rename = "const")]
    pub constant: ConstantConfig,
    /// Preprocessor defines to use when generating #ifdef's for #[cfg]
    pub defines: HashMap<String, String>,
    /// Include doc comments from rust as documentation
    pub documentation: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            header: None,
            includes: Vec::new(),
            sys_includes: Vec::new(),
            trailer: None,
            include_guard: None,
            autogen_warning: None,
            include_version: false,
            no_includes: false,
            namespace: None,
            namespaces: None,
            braces: Braces::SameLine,
            line_length: 100,
            tab_width: 2,
            language: Language::Cxx,
            style: Style::Type,
            parse: ParseConfig::default(),
            export: ExportConfig::default(),
            function: FunctionConfig::default(),
            structure: StructConfig::default(),
            enumeration: EnumConfig::default(),
            constant: ConstantConfig::default(),
            defines: HashMap::new(),
            documentation: true,
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
            Err(e) => Err(format!("Couldn't parse config file: {}.", e)),
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
