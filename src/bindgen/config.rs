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

// Warning: Do not change this without regenerating serde_derive impls.
// See issue #203 and README.serde_derive for more information.
//
/// Settings to apply when exporting items.
#[derive(Debug, Clone)]
#[cfg_attr(serde_derive, serde(rename_all = "snake_case"))]
#[cfg_attr(serde_derive, serde(deny_unknown_fields))]
#[cfg_attr(serde_derive, serde(default))]
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

// Warning: Do not change this without regenerating serde_derive impls.
// See issue #203 and README.serde_derive for more information.
//
/// Settings to apply to generated functions.
#[derive(Debug, Clone)]
#[cfg_attr(serde_derive, serde(rename_all = "snake_case"))]
#[cfg_attr(serde_derive, serde(deny_unknown_fields))]
#[cfg_attr(serde_derive, serde(default))]
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

// Warning: Do not change this without regenerating serde_derive impls.
// See issue #203 and README.serde_derive for more information.
//
/// Settings to apply to generated structs.
#[derive(Debug, Clone)]
#[cfg_attr(serde_derive, serde(rename_all = "snake_case"))]
#[cfg_attr(serde_derive, serde(deny_unknown_fields))]
#[cfg_attr(serde_derive, serde(default))]
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

// Warning: Do not change this without regenerating serde_derive impls.
// See issue #203 and README.serde_derive for more information.
//
/// Settings to apply to generated enums.
#[derive(Debug, Clone)]
#[cfg_attr(serde_derive, serde(rename_all = "snake_case"))]
#[cfg_attr(serde_derive, serde(deny_unknown_fields))]
#[cfg_attr(serde_derive, serde(default))]
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

// Warning: Do not change this without regenerating serde_derive impls.
// See issue #203 and README.serde_derive for more information.
//
/// Settings to apply to generated constants.
#[derive(Debug, Clone)]
#[cfg_attr(serde_derive, serde(rename_all = "snake_case"))]
#[cfg_attr(serde_derive, serde(deny_unknown_fields))]
#[cfg_attr(serde_derive, serde(default))]
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

// Warning: Do not change this without regenerating serde_derive impls.
// See issue #203 and README.serde_derive for more information.
//
/// Settings to apply when running `rustc --pretty=expanded`
#[derive(Debug, Clone)]
#[cfg_attr(serde_derive, serde(rename_all = "snake_case"))]
#[cfg_attr(serde_derive, serde(deny_unknown_fields))]
#[cfg_attr(serde_derive, serde(default))]
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

// Warning: Do not change this without regenerating serde_derive impls.
// See issue #203 and README.serde_derive for more information.
//
/// Settings to apply when parsing.
#[derive(Debug, Clone)]
#[cfg_attr(serde_derive, serde(rename_all = "snake_case"))]
#[cfg_attr(serde_derive, serde(deny_unknown_fields))]
#[cfg_attr(serde_derive, serde(default))]
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
    #[cfg_attr(serde_derive, serde(deserialize_with = "retrocomp_parse_expand_config_deserialize"))]
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

// Warning: Do not change this without regenerating serde_derive impls.
// See issue #203 and README.serde_derive for more information.
//
/// A collection of settings to customize the generated bindings.
#[derive(Debug, Clone)]
#[cfg_attr(serde_derive, serde(rename_all = "snake_case"))]
#[cfg_attr(serde_derive, serde(deny_unknown_fields))]
#[cfg_attr(serde_derive, serde(default))]
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
    #[cfg_attr(serde_derive, serde(rename = "fn"))]
    pub function: FunctionConfig,
    /// The configuration options for structs
    #[cfg_attr(serde_derive, serde(rename = "struct"))]
    pub structure: StructConfig,
    /// The configuration options for enums
    #[cfg_attr(serde_derive, serde(rename = "enum"))]
    pub enumeration: EnumConfig,
    /// The configuration options for constants
    #[cfg_attr(serde_derive, serde(rename = "const"))]
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

// Warning: The following code is autogenerated by serde_derive, don't touch
// unless you know what you're doing. See issue #203 and README.serde_derive
// for more information.

// Generated from `serde_derive 1.80.0`

#[allow(non_upper_case_globals,
        unused_attributes,
        unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_ExportConfig: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val ,
                         _serde :: export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err ) ; }
                         } });
        #[automatically_derived]
        impl <'de> _serde::Deserialize<'de> for ExportConfig where
         ExportConfig<>: _serde::export::Default {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                }
                struct __FieldVisitor;
                impl <'de> _serde::de::Visitor<'de> for __FieldVisitor
                 {
                    type
                    Value
                    =
                    __Field;
                    fn expecting(&self,
                                 __formatter:
                                     &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "field identifier")
                    }
                    fn visit_u64<__E>(self, __value: u64)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            0u64 =>
                            _serde::export::Ok(__Field::__field0),
                            1u64 =>
                            _serde::export::Ok(__Field::__field1),
                            2u64 =>
                            _serde::export::Ok(__Field::__field2),
                            3u64 =>
                            _serde::export::Ok(__Field::__field3),
                            4u64 =>
                            _serde::export::Ok(__Field::__field4),
                            _ =>
                            _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                 &"field index 0 <= i < 5")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            "include" =>
                            _serde::export::Ok(__Field::__field0),
                            "exclude" =>
                            _serde::export::Ok(__Field::__field1),
                            "rename" =>
                            _serde::export::Ok(__Field::__field2),
                            "prefix" =>
                            _serde::export::Ok(__Field::__field3),
                            "item_types" =>
                            _serde::export::Ok(__Field::__field4),
                            _ => {
                                _serde::export::Err(_serde::de::Error::unknown_field(__value,
                                                                                     FIELDS))
                            }
                        }
                    }
                    fn visit_bytes<__E>(self, __value: &[u8])
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            b"include" =>
                            _serde::export::Ok(__Field::__field0),
                            b"exclude" =>
                            _serde::export::Ok(__Field::__field1),
                            b"rename" =>
                            _serde::export::Ok(__Field::__field2),
                            b"prefix" =>
                            _serde::export::Ok(__Field::__field3),
                            b"item_types" =>
                            _serde::export::Ok(__Field::__field4),
                            _ => {
                                let __value =
                                    &_serde::export::from_utf8_lossy(__value);
                                _serde::export::Err(_serde::de::Error::unknown_field(__value,
                                                                                     FIELDS))
                            }
                        }
                    }
                }
                impl <'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(__deserializer: __D)
                     -> _serde::export::Result<Self, __D::Error> where
                     __D: _serde::Deserializer<'de> {
                        _serde::Deserializer::deserialize_identifier(__deserializer,
                                                                     __FieldVisitor)
                    }
                }
                struct __Visitor<'de> where
                       ExportConfig<>: _serde::export::Default {
                    marker: _serde::export::PhantomData<ExportConfig>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de> _serde::de::Visitor<'de> for __Visitor<'de>
                 where ExportConfig<>: _serde::export::Default {
                    type
                    Value
                    =
                    ExportConfig;
                    fn expecting(&self,
                                 __formatter:
                                     &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "struct ExportConfig")
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A)
                     ->
                         _serde::export::Result<Self::Value,
                                                __A::Error> where
                     __A: _serde::de::SeqAccess<'de> {
                        let __default: Self::Value =
                            _serde::export::Default::default();
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Vec<String>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"struct ExportConfig with 5 elements"));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<Vec<String>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                 &"struct ExportConfig with 5 elements"));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<HashMap<String,
                                                                                      String>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(2usize,
                                                                                                 &"struct ExportConfig with 5 elements"));
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<Option<String>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(3usize,
                                                                                                 &"struct ExportConfig with 5 elements"));
                                }
                            };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<Vec<ItemType>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(4usize,
                                                                                                 &"struct ExportConfig with 5 elements"));
                                }
                            };
                        _serde::export::Ok(ExportConfig{include:
                                                            __field0,
                                                        exclude:
                                                            __field1,
                                                        rename:
                                                            __field2,
                                                        prefix:
                                                            __field3,
                                                        item_types:
                                                            __field4,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     ->
                         _serde::export::Result<Self::Value,
                                                __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0:
                                _serde::export::Option<Vec<String>> =
                            _serde::export::None;
                        let mut __field1:
                                _serde::export::Option<Vec<String>> =
                            _serde::export::None;
                        let mut __field2:
                                _serde::export::Option<HashMap<String,
                                                               String>> =
                            _serde::export::None;
                        let mut __field3:
                                _serde::export::Option<Option<String>> =
                            _serde::export::None;
                        let mut __field4:
                                _serde::export::Option<Vec<ItemType>> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                                  match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("include"));
                                    }
                                    __field0 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Vec<String>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("exclude"));
                                    }
                                    __field1 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Vec<String>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("rename"));
                                    }
                                    __field2 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<HashMap<String,
                                                                                                               String>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("prefix"));
                                    }
                                    __field3 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<String>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field4 => {
                                    if _serde::export::Option::is_some(&__field4)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("item_types"));
                                    }
                                    __field4 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Vec<ItemType>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                            }
                        }
                        let __default: Self::Value =
                            _serde::export::Default::default();
                        let __field0 =
                            match __field0 {
                                _serde::export::Some(__field0) =>
                                __field0,
                                _serde::export::None =>
                                __default.include,
                            };
                        let __field1 =
                            match __field1 {
                                _serde::export::Some(__field1) =>
                                __field1,
                                _serde::export::None =>
                                __default.exclude,
                            };
                        let __field2 =
                            match __field2 {
                                _serde::export::Some(__field2) =>
                                __field2,
                                _serde::export::None =>
                                __default.rename,
                            };
                        let __field3 =
                            match __field3 {
                                _serde::export::Some(__field3) =>
                                __field3,
                                _serde::export::None =>
                                __default.prefix,
                            };
                        let __field4 =
                            match __field4 {
                                _serde::export::Some(__field4) =>
                                __field4,
                                _serde::export::None =>
                                __default.item_types,
                            };
                        _serde::export::Ok(ExportConfig{include:
                                                            __field0,
                                                        exclude:
                                                            __field1,
                                                        rename:
                                                            __field2,
                                                        prefix:
                                                            __field3,
                                                        item_types:
                                                            __field4,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["include", "exclude", "rename", "prefix",
                      "item_types"];
                _serde::Deserializer::deserialize_struct(__deserializer,
                                                         "ExportConfig",
                                                         FIELDS,
                                                         __Visitor{marker:
                                                                       _serde::export::PhantomData::<ExportConfig>,
                                                                   lifetime:
                                                                       _serde::export::PhantomData,})
            }
        }
    };

#[allow(non_upper_case_globals,
        unused_attributes,
        unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_FunctionConfig: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val ,
                         _serde :: export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err ) ; }
                         } });
        #[automatically_derived]
        impl <'de> _serde::Deserialize<'de> for FunctionConfig where
         FunctionConfig<>: _serde::export::Default {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                }
                struct __FieldVisitor;
                impl <'de> _serde::de::Visitor<'de> for __FieldVisitor
                 {
                    type
                    Value
                    =
                    __Field;
                    fn expecting(&self,
                                 __formatter:
                                     &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "field identifier")
                    }
                    fn visit_u64<__E>(self, __value: u64)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            0u64 =>
                            _serde::export::Ok(__Field::__field0),
                            1u64 =>
                            _serde::export::Ok(__Field::__field1),
                            2u64 =>
                            _serde::export::Ok(__Field::__field2),
                            3u64 =>
                            _serde::export::Ok(__Field::__field3),
                            _ =>
                            _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                 &"field index 0 <= i < 4")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            "prefix" =>
                            _serde::export::Ok(__Field::__field0),
                            "postfix" =>
                            _serde::export::Ok(__Field::__field1),
                            "args" =>
                            _serde::export::Ok(__Field::__field2),
                            "rename_args" =>
                            _serde::export::Ok(__Field::__field3),
                            _ => {
                                _serde::export::Err(_serde::de::Error::unknown_field(__value,
                                                                                     FIELDS))
                            }
                        }
                    }
                    fn visit_bytes<__E>(self, __value: &[u8])
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            b"prefix" =>
                            _serde::export::Ok(__Field::__field0),
                            b"postfix" =>
                            _serde::export::Ok(__Field::__field1),
                            b"args" =>
                            _serde::export::Ok(__Field::__field2),
                            b"rename_args" =>
                            _serde::export::Ok(__Field::__field3),
                            _ => {
                                let __value =
                                    &_serde::export::from_utf8_lossy(__value);
                                _serde::export::Err(_serde::de::Error::unknown_field(__value,
                                                                                     FIELDS))
                            }
                        }
                    }
                }
                impl <'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(__deserializer: __D)
                     -> _serde::export::Result<Self, __D::Error> where
                     __D: _serde::Deserializer<'de> {
                        _serde::Deserializer::deserialize_identifier(__deserializer,
                                                                     __FieldVisitor)
                    }
                }
                struct __Visitor<'de> where
                       FunctionConfig<>: _serde::export::Default {
                    marker: _serde::export::PhantomData<FunctionConfig>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de> _serde::de::Visitor<'de> for __Visitor<'de>
                 where FunctionConfig<>: _serde::export::Default {
                    type
                    Value
                    =
                    FunctionConfig;
                    fn expecting(&self,
                                 __formatter:
                                     &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "struct FunctionConfig")
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A)
                     ->
                         _serde::export::Result<Self::Value,
                                                __A::Error> where
                     __A: _serde::de::SeqAccess<'de> {
                        let __default: Self::Value =
                            _serde::export::Default::default();
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Option<String>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"struct FunctionConfig with 4 elements"));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<Option<String>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                 &"struct FunctionConfig with 4 elements"));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<Layout>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(2usize,
                                                                                                 &"struct FunctionConfig with 4 elements"));
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<Option<RenameRule>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(3usize,
                                                                                                 &"struct FunctionConfig with 4 elements"));
                                }
                            };
                        _serde::export::Ok(FunctionConfig{prefix:
                                                              __field0,
                                                          postfix:
                                                              __field1,
                                                          args:
                                                              __field2,
                                                          rename_args:
                                                              __field3,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     ->
                         _serde::export::Result<Self::Value,
                                                __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0:
                                _serde::export::Option<Option<String>> =
                            _serde::export::None;
                        let mut __field1:
                                _serde::export::Option<Option<String>> =
                            _serde::export::None;
                        let mut __field2:
                                _serde::export::Option<Layout> =
                            _serde::export::None;
                        let mut __field3:
                                _serde::export::Option<Option<RenameRule>> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                                  match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("prefix"));
                                    }
                                    __field0 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<String>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("postfix"));
                                    }
                                    __field1 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<String>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("args"));
                                    }
                                    __field2 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Layout>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("rename_args"));
                                    }
                                    __field3 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<RenameRule>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                            }
                        }
                        let __default: Self::Value =
                            _serde::export::Default::default();
                        let __field0 =
                            match __field0 {
                                _serde::export::Some(__field0) =>
                                __field0,
                                _serde::export::None =>
                                __default.prefix,
                            };
                        let __field1 =
                            match __field1 {
                                _serde::export::Some(__field1) =>
                                __field1,
                                _serde::export::None =>
                                __default.postfix,
                            };
                        let __field2 =
                            match __field2 {
                                _serde::export::Some(__field2) =>
                                __field2,
                                _serde::export::None =>
                                __default.args,
                            };
                        let __field3 =
                            match __field3 {
                                _serde::export::Some(__field3) =>
                                __field3,
                                _serde::export::None =>
                                __default.rename_args,
                            };
                        _serde::export::Ok(FunctionConfig{prefix:
                                                              __field0,
                                                          postfix:
                                                              __field1,
                                                          args:
                                                              __field2,
                                                          rename_args:
                                                              __field3,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["prefix", "postfix", "args", "rename_args"];
                _serde::Deserializer::deserialize_struct(__deserializer,
                                                         "FunctionConfig",
                                                         FIELDS,
                                                         __Visitor{marker:
                                                                       _serde::export::PhantomData::<FunctionConfig>,
                                                                   lifetime:
                                                                       _serde::export::PhantomData,})
            }
        }
    };
#[allow(non_upper_case_globals,
        unused_attributes,
        unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_StructConfig: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val ,
                         _serde :: export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err ) ; }
                         } });
        #[automatically_derived]
        impl <'de> _serde::Deserialize<'de> for StructConfig where
         StructConfig<>: _serde::export::Default {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                }
                struct __FieldVisitor;
                impl <'de> _serde::de::Visitor<'de> for __FieldVisitor
                 {
                    type
                    Value
                    =
                    __Field;
                    fn expecting(&self,
                                 __formatter:
                                     &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "field identifier")
                    }
                    fn visit_u64<__E>(self, __value: u64)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            0u64 =>
                            _serde::export::Ok(__Field::__field0),
                            1u64 =>
                            _serde::export::Ok(__Field::__field1),
                            2u64 =>
                            _serde::export::Ok(__Field::__field2),
                            3u64 =>
                            _serde::export::Ok(__Field::__field3),
                            4u64 =>
                            _serde::export::Ok(__Field::__field4),
                            5u64 =>
                            _serde::export::Ok(__Field::__field5),
                            6u64 =>
                            _serde::export::Ok(__Field::__field6),
                            7u64 =>
                            _serde::export::Ok(__Field::__field7),
                            _ =>
                            _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                 &"field index 0 <= i < 8")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            "rename_fields" =>
                            _serde::export::Ok(__Field::__field0),
                            "derive_constructor" =>
                            _serde::export::Ok(__Field::__field1),
                            "derive_eq" =>
                            _serde::export::Ok(__Field::__field2),
                            "derive_neq" =>
                            _serde::export::Ok(__Field::__field3),
                            "derive_lt" =>
                            _serde::export::Ok(__Field::__field4),
                            "derive_lte" =>
                            _serde::export::Ok(__Field::__field5),
                            "derive_gt" =>
                            _serde::export::Ok(__Field::__field6),
                            "derive_gte" =>
                            _serde::export::Ok(__Field::__field7),
                            _ => {
                                _serde::export::Err(_serde::de::Error::unknown_field(__value,
                                                                                     FIELDS))
                            }
                        }
                    }
                    fn visit_bytes<__E>(self, __value: &[u8])
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            b"rename_fields" =>
                            _serde::export::Ok(__Field::__field0),
                            b"derive_constructor" =>
                            _serde::export::Ok(__Field::__field1),
                            b"derive_eq" =>
                            _serde::export::Ok(__Field::__field2),
                            b"derive_neq" =>
                            _serde::export::Ok(__Field::__field3),
                            b"derive_lt" =>
                            _serde::export::Ok(__Field::__field4),
                            b"derive_lte" =>
                            _serde::export::Ok(__Field::__field5),
                            b"derive_gt" =>
                            _serde::export::Ok(__Field::__field6),
                            b"derive_gte" =>
                            _serde::export::Ok(__Field::__field7),
                            _ => {
                                let __value =
                                    &_serde::export::from_utf8_lossy(__value);
                                _serde::export::Err(_serde::de::Error::unknown_field(__value,
                                                                                     FIELDS))
                            }
                        }
                    }
                }
                impl <'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(__deserializer: __D)
                     -> _serde::export::Result<Self, __D::Error> where
                     __D: _serde::Deserializer<'de> {
                        _serde::Deserializer::deserialize_identifier(__deserializer,
                                                                     __FieldVisitor)
                    }
                }
                struct __Visitor<'de> where
                       StructConfig<>: _serde::export::Default {
                    marker: _serde::export::PhantomData<StructConfig>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de> _serde::de::Visitor<'de> for __Visitor<'de>
                 where StructConfig<>: _serde::export::Default {
                    type
                    Value
                    =
                    StructConfig;
                    fn expecting(&self,
                                 __formatter:
                                     &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "struct StructConfig")
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A)
                     ->
                         _serde::export::Result<Self::Value,
                                                __A::Error> where
                     __A: _serde::de::SeqAccess<'de> {
                        let __default: Self::Value =
                            _serde::export::Default::default();
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Option<RenameRule>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"struct StructConfig with 8 elements"));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                 &"struct StructConfig with 8 elements"));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(2usize,
                                                                                                 &"struct StructConfig with 8 elements"));
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(3usize,
                                                                                                 &"struct StructConfig with 8 elements"));
                                }
                            };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(4usize,
                                                                                                 &"struct StructConfig with 8 elements"));
                                }
                            };
                        let __field5 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(5usize,
                                                                                                 &"struct StructConfig with 8 elements"));
                                }
                            };
                        let __field6 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(6usize,
                                                                                                 &"struct StructConfig with 8 elements"));
                                }
                            };
                        let __field7 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(7usize,
                                                                                                 &"struct StructConfig with 8 elements"));
                                }
                            };
                        _serde::export::Ok(StructConfig{rename_fields:
                                                            __field0,
                                                        derive_constructor:
                                                            __field1,
                                                        derive_eq:
                                                            __field2,
                                                        derive_neq:
                                                            __field3,
                                                        derive_lt:
                                                            __field4,
                                                        derive_lte:
                                                            __field5,
                                                        derive_gt:
                                                            __field6,
                                                        derive_gte:
                                                            __field7,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     ->
                         _serde::export::Result<Self::Value,
                                                __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0:
                                _serde::export::Option<Option<RenameRule>> =
                            _serde::export::None;
                        let mut __field1:
                                _serde::export::Option<bool> =
                            _serde::export::None;
                        let mut __field2:
                                _serde::export::Option<bool> =
                            _serde::export::None;
                        let mut __field3:
                                _serde::export::Option<bool> =
                            _serde::export::None;
                        let mut __field4:
                                _serde::export::Option<bool> =
                            _serde::export::None;
                        let mut __field5:
                                _serde::export::Option<bool> =
                            _serde::export::None;
                        let mut __field6:
                                _serde::export::Option<bool> =
                            _serde::export::None;
                        let mut __field7:
                                _serde::export::Option<bool> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                                  match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("rename_fields"));
                                    }
                                    __field0 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<RenameRule>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("derive_constructor"));
                                    }
                                    __field1 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("derive_eq"));
                                    }
                                    __field2 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("derive_neq"));
                                    }
                                    __field3 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field4 => {
                                    if _serde::export::Option::is_some(&__field4)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("derive_lt"));
                                    }
                                    __field4 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field5 => {
                                    if _serde::export::Option::is_some(&__field5)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("derive_lte"));
                                    }
                                    __field5 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field6 => {
                                    if _serde::export::Option::is_some(&__field6)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("derive_gt"));
                                    }
                                    __field6 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field7 => {
                                    if _serde::export::Option::is_some(&__field7)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("derive_gte"));
                                    }
                                    __field7 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                            }
                        }
                        let __default: Self::Value =
                            _serde::export::Default::default();
                        let __field0 =
                            match __field0 {
                                _serde::export::Some(__field0) =>
                                __field0,
                                _serde::export::None =>
                                __default.rename_fields,
                            };
                        let __field1 =
                            match __field1 {
                                _serde::export::Some(__field1) =>
                                __field1,
                                _serde::export::None =>
                                __default.derive_constructor,
                            };
                        let __field2 =
                            match __field2 {
                                _serde::export::Some(__field2) =>
                                __field2,
                                _serde::export::None =>
                                __default.derive_eq,
                            };
                        let __field3 =
                            match __field3 {
                                _serde::export::Some(__field3) =>
                                __field3,
                                _serde::export::None =>
                                __default.derive_neq,
                            };
                        let __field4 =
                            match __field4 {
                                _serde::export::Some(__field4) =>
                                __field4,
                                _serde::export::None =>
                                __default.derive_lt,
                            };
                        let __field5 =
                            match __field5 {
                                _serde::export::Some(__field5) =>
                                __field5,
                                _serde::export::None =>
                                __default.derive_lte,
                            };
                        let __field6 =
                            match __field6 {
                                _serde::export::Some(__field6) =>
                                __field6,
                                _serde::export::None =>
                                __default.derive_gt,
                            };
                        let __field7 =
                            match __field7 {
                                _serde::export::Some(__field7) =>
                                __field7,
                                _serde::export::None =>
                                __default.derive_gte,
                            };
                        _serde::export::Ok(StructConfig{rename_fields:
                                                            __field0,
                                                        derive_constructor:
                                                            __field1,
                                                        derive_eq:
                                                            __field2,
                                                        derive_neq:
                                                            __field3,
                                                        derive_lt:
                                                            __field4,
                                                        derive_lte:
                                                            __field5,
                                                        derive_gt:
                                                            __field6,
                                                        derive_gte:
                                                            __field7,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["rename_fields", "derive_constructor",
                      "derive_eq", "derive_neq", "derive_lt",
                      "derive_lte", "derive_gt", "derive_gte"];
                _serde::Deserializer::deserialize_struct(__deserializer,
                                                         "StructConfig",
                                                         FIELDS,
                                                         __Visitor{marker:
                                                                       _serde::export::PhantomData::<StructConfig>,
                                                                   lifetime:
                                                                       _serde::export::PhantomData,})
            }
        }
    };
#[allow(non_upper_case_globals,
        unused_attributes,
        unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_EnumConfig: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val ,
                         _serde :: export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err ) ; }
                         } });
        #[automatically_derived]
        impl <'de> _serde::Deserialize<'de> for EnumConfig where
         EnumConfig<>: _serde::export::Default {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                }
                struct __FieldVisitor;
                impl <'de> _serde::de::Visitor<'de> for __FieldVisitor
                 {
                    type
                    Value
                    =
                    __Field;
                    fn expecting(&self,
                                 __formatter:
                                     &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "field identifier")
                    }
                    fn visit_u64<__E>(self, __value: u64)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            0u64 =>
                            _serde::export::Ok(__Field::__field0),
                            1u64 =>
                            _serde::export::Ok(__Field::__field1),
                            2u64 =>
                            _serde::export::Ok(__Field::__field2),
                            3u64 =>
                            _serde::export::Ok(__Field::__field3),
                            _ =>
                            _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                 &"field index 0 <= i < 4")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            "rename_variants" =>
                            _serde::export::Ok(__Field::__field0),
                            "add_sentinel" =>
                            _serde::export::Ok(__Field::__field1),
                            "prefix_with_name" =>
                            _serde::export::Ok(__Field::__field2),
                            "derive_helper_methods" =>
                            _serde::export::Ok(__Field::__field3),
                            _ => {
                                _serde::export::Err(_serde::de::Error::unknown_field(__value,
                                                                                     FIELDS))
                            }
                        }
                    }
                    fn visit_bytes<__E>(self, __value: &[u8])
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            b"rename_variants" =>
                            _serde::export::Ok(__Field::__field0),
                            b"add_sentinel" =>
                            _serde::export::Ok(__Field::__field1),
                            b"prefix_with_name" =>
                            _serde::export::Ok(__Field::__field2),
                            b"derive_helper_methods" =>
                            _serde::export::Ok(__Field::__field3),
                            _ => {
                                let __value =
                                    &_serde::export::from_utf8_lossy(__value);
                                _serde::export::Err(_serde::de::Error::unknown_field(__value,
                                                                                     FIELDS))
                            }
                        }
                    }
                }
                impl <'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(__deserializer: __D)
                     -> _serde::export::Result<Self, __D::Error> where
                     __D: _serde::Deserializer<'de> {
                        _serde::Deserializer::deserialize_identifier(__deserializer,
                                                                     __FieldVisitor)
                    }
                }
                struct __Visitor<'de> where
                       EnumConfig<>: _serde::export::Default {
                    marker: _serde::export::PhantomData<EnumConfig>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de> _serde::de::Visitor<'de> for __Visitor<'de>
                 where EnumConfig<>: _serde::export::Default {
                    type
                    Value
                    =
                    EnumConfig;
                    fn expecting(&self,
                                 __formatter:
                                     &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "struct EnumConfig")
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A)
                     ->
                         _serde::export::Result<Self::Value,
                                                __A::Error> where
                     __A: _serde::de::SeqAccess<'de> {
                        let __default: Self::Value =
                            _serde::export::Default::default();
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Option<RenameRule>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"struct EnumConfig with 4 elements"));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                 &"struct EnumConfig with 4 elements"));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(2usize,
                                                                                                 &"struct EnumConfig with 4 elements"));
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(3usize,
                                                                                                 &"struct EnumConfig with 4 elements"));
                                }
                            };
                        _serde::export::Ok(EnumConfig{rename_variants:
                                                          __field0,
                                                      add_sentinel:
                                                          __field1,
                                                      prefix_with_name:
                                                          __field2,
                                                      derive_helper_methods:
                                                          __field3,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     ->
                         _serde::export::Result<Self::Value,
                                                __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0:
                                _serde::export::Option<Option<RenameRule>> =
                            _serde::export::None;
                        let mut __field1:
                                _serde::export::Option<bool> =
                            _serde::export::None;
                        let mut __field2:
                                _serde::export::Option<bool> =
                            _serde::export::None;
                        let mut __field3:
                                _serde::export::Option<bool> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                                  match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("rename_variants"));
                                    }
                                    __field0 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<RenameRule>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("add_sentinel"));
                                    }
                                    __field1 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("prefix_with_name"));
                                    }
                                    __field2 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("derive_helper_methods"));
                                    }
                                    __field3 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                            }
                        }
                        let __default: Self::Value =
                            _serde::export::Default::default();
                        let __field0 =
                            match __field0 {
                                _serde::export::Some(__field0) =>
                                __field0,
                                _serde::export::None =>
                                __default.rename_variants,
                            };
                        let __field1 =
                            match __field1 {
                                _serde::export::Some(__field1) =>
                                __field1,
                                _serde::export::None =>
                                __default.add_sentinel,
                            };
                        let __field2 =
                            match __field2 {
                                _serde::export::Some(__field2) =>
                                __field2,
                                _serde::export::None =>
                                __default.prefix_with_name,
                            };
                        let __field3 =
                            match __field3 {
                                _serde::export::Some(__field3) =>
                                __field3,
                                _serde::export::None =>
                                __default.derive_helper_methods,
                            };
                        _serde::export::Ok(EnumConfig{rename_variants:
                                                          __field0,
                                                      add_sentinel:
                                                          __field1,
                                                      prefix_with_name:
                                                          __field2,
                                                      derive_helper_methods:
                                                          __field3,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["rename_variants", "add_sentinel",
                      "prefix_with_name", "derive_helper_methods"];
                _serde::Deserializer::deserialize_struct(__deserializer,
                                                         "EnumConfig",
                                                         FIELDS,
                                                         __Visitor{marker:
                                                                       _serde::export::PhantomData::<EnumConfig>,
                                                                   lifetime:
                                                                       _serde::export::PhantomData,})
            }
        }
    };
#[allow(non_upper_case_globals,
        unused_attributes,
        unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_ConstantConfig: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val ,
                         _serde :: export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err ) ; }
                         } });
        #[automatically_derived]
        impl <'de> _serde::Deserialize<'de> for ConstantConfig where
         ConstantConfig<>: _serde::export::Default {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                #[allow(non_camel_case_types)]
                enum __Field { __field0, }
                struct __FieldVisitor;
                impl <'de> _serde::de::Visitor<'de> for __FieldVisitor
                 {
                    type
                    Value
                    =
                    __Field;
                    fn expecting(&self,
                                 __formatter:
                                     &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "field identifier")
                    }
                    fn visit_u64<__E>(self, __value: u64)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            0u64 =>
                            _serde::export::Ok(__Field::__field0),
                            _ =>
                            _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                 &"field index 0 <= i < 1")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            "allow_static_const" =>
                            _serde::export::Ok(__Field::__field0),
                            _ => {
                                _serde::export::Err(_serde::de::Error::unknown_field(__value,
                                                                                     FIELDS))
                            }
                        }
                    }
                    fn visit_bytes<__E>(self, __value: &[u8])
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            b"allow_static_const" =>
                            _serde::export::Ok(__Field::__field0),
                            _ => {
                                let __value =
                                    &_serde::export::from_utf8_lossy(__value);
                                _serde::export::Err(_serde::de::Error::unknown_field(__value,
                                                                                     FIELDS))
                            }
                        }
                    }
                }
                impl <'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(__deserializer: __D)
                     -> _serde::export::Result<Self, __D::Error> where
                     __D: _serde::Deserializer<'de> {
                        _serde::Deserializer::deserialize_identifier(__deserializer,
                                                                     __FieldVisitor)
                    }
                }
                struct __Visitor<'de> where
                       ConstantConfig<>: _serde::export::Default {
                    marker: _serde::export::PhantomData<ConstantConfig>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de> _serde::de::Visitor<'de> for __Visitor<'de>
                 where ConstantConfig<>: _serde::export::Default {
                    type
                    Value
                    =
                    ConstantConfig;
                    fn expecting(&self,
                                 __formatter:
                                     &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "struct ConstantConfig")
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A)
                     ->
                         _serde::export::Result<Self::Value,
                                                __A::Error> where
                     __A: _serde::de::SeqAccess<'de> {
                        let __default: Self::Value =
                            _serde::export::Default::default();
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"struct ConstantConfig with 1 element"));
                                }
                            };
                        _serde::export::Ok(ConstantConfig{allow_static_const:
                                                              __field0,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     ->
                         _serde::export::Result<Self::Value,
                                                __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0:
                                _serde::export::Option<bool> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                                  match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("allow_static_const"));
                                    }
                                    __field0 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                            }
                        }
                        let __default: Self::Value =
                            _serde::export::Default::default();
                        let __field0 =
                            match __field0 {
                                _serde::export::Some(__field0) =>
                                __field0,
                                _serde::export::None =>
                                __default.allow_static_const,
                            };
                        _serde::export::Ok(ConstantConfig{allow_static_const:
                                                              __field0,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["allow_static_const"];
                _serde::Deserializer::deserialize_struct(__deserializer,
                                                         "ConstantConfig",
                                                         FIELDS,
                                                         __Visitor{marker:
                                                                       _serde::export::PhantomData::<ConstantConfig>,
                                                                   lifetime:
                                                                       _serde::export::PhantomData,})
            }
        }
    };
#[allow(non_upper_case_globals,
        unused_attributes,
        unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_ParseExpandConfig: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val ,
                         _serde :: export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err ) ; }
                         } });
        #[automatically_derived]
        impl <'de> _serde::Deserialize<'de> for ParseExpandConfig
         where ParseExpandConfig<>: _serde::export::Default {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                }
                struct __FieldVisitor;
                impl <'de> _serde::de::Visitor<'de> for __FieldVisitor
                 {
                    type
                    Value
                    =
                    __Field;
                    fn expecting(&self,
                                 __formatter:
                                     &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "field identifier")
                    }
                    fn visit_u64<__E>(self, __value: u64)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            0u64 =>
                            _serde::export::Ok(__Field::__field0),
                            1u64 =>
                            _serde::export::Ok(__Field::__field1),
                            2u64 =>
                            _serde::export::Ok(__Field::__field2),
                            3u64 =>
                            _serde::export::Ok(__Field::__field3),
                            _ =>
                            _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                 &"field index 0 <= i < 4")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            "crates" =>
                            _serde::export::Ok(__Field::__field0),
                            "all_features" =>
                            _serde::export::Ok(__Field::__field1),
                            "default_features" =>
                            _serde::export::Ok(__Field::__field2),
                            "features" =>
                            _serde::export::Ok(__Field::__field3),
                            _ => {
                                _serde::export::Err(_serde::de::Error::unknown_field(__value,
                                                                                     FIELDS))
                            }
                        }
                    }
                    fn visit_bytes<__E>(self, __value: &[u8])
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            b"crates" =>
                            _serde::export::Ok(__Field::__field0),
                            b"all_features" =>
                            _serde::export::Ok(__Field::__field1),
                            b"default_features" =>
                            _serde::export::Ok(__Field::__field2),
                            b"features" =>
                            _serde::export::Ok(__Field::__field3),
                            _ => {
                                let __value =
                                    &_serde::export::from_utf8_lossy(__value);
                                _serde::export::Err(_serde::de::Error::unknown_field(__value,
                                                                                     FIELDS))
                            }
                        }
                    }
                }
                impl <'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(__deserializer: __D)
                     -> _serde::export::Result<Self, __D::Error> where
                     __D: _serde::Deserializer<'de> {
                        _serde::Deserializer::deserialize_identifier(__deserializer,
                                                                     __FieldVisitor)
                    }
                }
                struct __Visitor<'de> where
                       ParseExpandConfig<>: _serde::export::Default {
                    marker: _serde::export::PhantomData<ParseExpandConfig>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de> _serde::de::Visitor<'de> for __Visitor<'de>
                 where ParseExpandConfig<>: _serde::export::Default {
                    type
                    Value
                    =
                    ParseExpandConfig;
                    fn expecting(&self,
                                 __formatter:
                                     &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "struct ParseExpandConfig")
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A)
                     ->
                         _serde::export::Result<Self::Value,
                                                __A::Error> where
                     __A: _serde::de::SeqAccess<'de> {
                        let __default: Self::Value =
                            _serde::export::Default::default();
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Vec<String>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"struct ParseExpandConfig with 4 elements"));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                 &"struct ParseExpandConfig with 4 elements"));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(2usize,
                                                                                                 &"struct ParseExpandConfig with 4 elements"));
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<Option<Vec<String>>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(3usize,
                                                                                                 &"struct ParseExpandConfig with 4 elements"));
                                }
                            };
                        _serde::export::Ok(ParseExpandConfig{crates:
                                                                 __field0,
                                                             all_features:
                                                                 __field1,
                                                             default_features:
                                                                 __field2,
                                                             features:
                                                                 __field3,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     ->
                         _serde::export::Result<Self::Value,
                                                __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0:
                                _serde::export::Option<Vec<String>> =
                            _serde::export::None;
                        let mut __field1:
                                _serde::export::Option<bool> =
                            _serde::export::None;
                        let mut __field2:
                                _serde::export::Option<bool> =
                            _serde::export::None;
                        let mut __field3:
                                _serde::export::Option<Option<Vec<String>>> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                                  match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("crates"));
                                    }
                                    __field0 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Vec<String>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("all_features"));
                                    }
                                    __field1 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("default_features"));
                                    }
                                    __field2 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("features"));
                                    }
                                    __field3 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<Vec<String>>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                            }
                        }
                        let __default: Self::Value =
                            _serde::export::Default::default();
                        let __field0 =
                            match __field0 {
                                _serde::export::Some(__field0) =>
                                __field0,
                                _serde::export::None =>
                                __default.crates,
                            };
                        let __field1 =
                            match __field1 {
                                _serde::export::Some(__field1) =>
                                __field1,
                                _serde::export::None =>
                                __default.all_features,
                            };
                        let __field2 =
                            match __field2 {
                                _serde::export::Some(__field2) =>
                                __field2,
                                _serde::export::None =>
                                __default.default_features,
                            };
                        let __field3 =
                            match __field3 {
                                _serde::export::Some(__field3) =>
                                __field3,
                                _serde::export::None =>
                                __default.features,
                            };
                        _serde::export::Ok(ParseExpandConfig{crates:
                                                                 __field0,
                                                             all_features:
                                                                 __field1,
                                                             default_features:
                                                                 __field2,
                                                             features:
                                                                 __field3,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["crates", "all_features", "default_features",
                      "features"];
                _serde::Deserializer::deserialize_struct(__deserializer,
                                                         "ParseExpandConfig",
                                                         FIELDS,
                                                         __Visitor{marker:
                                                                       _serde::export::PhantomData::<ParseExpandConfig>,
                                                                   lifetime:
                                                                       _serde::export::PhantomData,})
            }
        }
    };
#[allow(non_upper_case_globals,
        unused_attributes,
        unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_ParseConfig: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val ,
                         _serde :: export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err ) ; }
                         } });
        #[automatically_derived]
        impl <'de> _serde::Deserialize<'de> for ParseConfig where
         ParseConfig<>: _serde::export::Default {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                }
                struct __FieldVisitor;
                impl <'de> _serde::de::Visitor<'de> for __FieldVisitor
                 {
                    type
                    Value
                    =
                    __Field;
                    fn expecting(&self,
                                 __formatter:
                                     &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "field identifier")
                    }
                    fn visit_u64<__E>(self, __value: u64)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            0u64 =>
                            _serde::export::Ok(__Field::__field0),
                            1u64 =>
                            _serde::export::Ok(__Field::__field1),
                            2u64 =>
                            _serde::export::Ok(__Field::__field2),
                            3u64 =>
                            _serde::export::Ok(__Field::__field3),
                            4u64 =>
                            _serde::export::Ok(__Field::__field4),
                            _ =>
                            _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                 &"field index 0 <= i < 5")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            "parse_deps" =>
                            _serde::export::Ok(__Field::__field0),
                            "include" =>
                            _serde::export::Ok(__Field::__field1),
                            "exclude" =>
                            _serde::export::Ok(__Field::__field2),
                            "expand" =>
                            _serde::export::Ok(__Field::__field3),
                            "clean" =>
                            _serde::export::Ok(__Field::__field4),
                            _ => {
                                _serde::export::Err(_serde::de::Error::unknown_field(__value,
                                                                                     FIELDS))
                            }
                        }
                    }
                    fn visit_bytes<__E>(self, __value: &[u8])
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            b"parse_deps" =>
                            _serde::export::Ok(__Field::__field0),
                            b"include" =>
                            _serde::export::Ok(__Field::__field1),
                            b"exclude" =>
                            _serde::export::Ok(__Field::__field2),
                            b"expand" =>
                            _serde::export::Ok(__Field::__field3),
                            b"clean" =>
                            _serde::export::Ok(__Field::__field4),
                            _ => {
                                let __value =
                                    &_serde::export::from_utf8_lossy(__value);
                                _serde::export::Err(_serde::de::Error::unknown_field(__value,
                                                                                     FIELDS))
                            }
                        }
                    }
                }
                impl <'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(__deserializer: __D)
                     -> _serde::export::Result<Self, __D::Error> where
                     __D: _serde::Deserializer<'de> {
                        _serde::Deserializer::deserialize_identifier(__deserializer,
                                                                     __FieldVisitor)
                    }
                }
                struct __Visitor<'de> where
                       ParseConfig<>: _serde::export::Default {
                    marker: _serde::export::PhantomData<ParseConfig>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de> _serde::de::Visitor<'de> for __Visitor<'de>
                 where ParseConfig<>: _serde::export::Default {
                    type
                    Value
                    =
                    ParseConfig;
                    fn expecting(&self,
                                 __formatter:
                                     &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "struct ParseConfig")
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A)
                     ->
                         _serde::export::Result<Self::Value,
                                                __A::Error> where
                     __A: _serde::de::SeqAccess<'de> {
                        let __default: Self::Value =
                            _serde::export::Default::default();
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"struct ParseConfig with 5 elements"));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<Option<Vec<String>>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                 &"struct ParseConfig with 5 elements"));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<Vec<String>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(2usize,
                                                                                                 &"struct ParseConfig with 5 elements"));
                                }
                            };
                        let __field3 =
                            match {
                                      struct __DeserializeWith<'de>
                                             where
                                             ParseConfig<>: _serde::export::Default {
                                          value: ParseExpandConfig,
                                          phantom: _serde::export::PhantomData<ParseConfig>,
                                          lifetime: _serde::export::PhantomData<&'de ()>,
                                      }
                                      impl <'de>
                                       _serde::Deserialize<'de> for
                                       __DeserializeWith<'de> where
                                       ParseConfig<>: _serde::export::Default
                                       {
                                          fn deserialize<__D>(__deserializer:
                                                                  __D)
                                           ->
                                               _serde::export::Result<Self,
                                                                      __D::Error>
                                           where
                                           __D: _serde::Deserializer<'de> {
                                              _serde::export::Ok(__DeserializeWith{value:
                                                                                       match retrocomp_parse_expand_config_deserialize(__deserializer)
                                                                                           {
                                                                                           _serde::export::Ok(__val)
                                                                                           =>
                                                                                           __val,
                                                                                           _serde::export::Err(__err)
                                                                                           =>
                                                                                           {
                                                                                               return _serde::export::Err(__err);
                                                                                           }
                                                                                       },
                                                                                   phantom:
                                                                                       _serde::export::PhantomData,
                                                                                   lifetime:
                                                                                       _serde::export::PhantomData,})
                                          }
                                      }
                                      _serde::export::Option::map(match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(&mut __seq)
                                                                      {
                                                                      _serde::export::Ok(__val)
                                                                      =>
                                                                      __val,
                                                                      _serde::export::Err(__err)
                                                                      =>
                                                                      {
                                                                          return _serde::export::Err(__err);
                                                                      }
                                                                  },
                                                                  |__wrap|
                                                                      __wrap.value)
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(3usize,
                                                                                                 &"struct ParseConfig with 5 elements"));
                                }
                            };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(4usize,
                                                                                                 &"struct ParseConfig with 5 elements"));
                                }
                            };
                        _serde::export::Ok(ParseConfig{parse_deps:
                                                           __field0,
                                                       include:
                                                           __field1,
                                                       exclude:
                                                           __field2,
                                                       expand:
                                                           __field3,
                                                       clean:
                                                           __field4,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     ->
                         _serde::export::Result<Self::Value,
                                                __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0:
                                _serde::export::Option<bool> =
                            _serde::export::None;
                        let mut __field1:
                                _serde::export::Option<Option<Vec<String>>> =
                            _serde::export::None;
                        let mut __field2:
                                _serde::export::Option<Vec<String>> =
                            _serde::export::None;
                        let mut __field3:
                                _serde::export::Option<ParseExpandConfig> =
                            _serde::export::None;
                        let mut __field4:
                                _serde::export::Option<bool> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                                  match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("parse_deps"));
                                    }
                                    __field0 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("include"));
                                    }
                                    __field1 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<Vec<String>>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("exclude"));
                                    }
                                    __field2 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Vec<String>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("expand"));
                                    }
                                    __field3 =
                                        _serde::export::Some({
                                                                 struct __DeserializeWith<'de>
                                                                        where
                                                                        ParseConfig<>: _serde::export::Default {
                                                                     value: ParseExpandConfig,
                                                                     phantom: _serde::export::PhantomData<ParseConfig>,
                                                                     lifetime: _serde::export::PhantomData<&'de ()>,
                                                                 }
                                                                 impl <'de>
                                                                  _serde::Deserialize<'de>
                                                                  for
                                                                  __DeserializeWith<'de>
                                                                  where
                                                                  ParseConfig<>: _serde::export::Default
                                                                  {
                                                                     fn deserialize<__D>(__deserializer:
                                                                                             __D)
                                                                      ->
                                                                          _serde::export::Result<Self,
                                                                                                 __D::Error>
                                                                      where
                                                                      __D: _serde::Deserializer<'de> {
                                                                         _serde::export::Ok(__DeserializeWith{value:
                                                                                                                  match retrocomp_parse_expand_config_deserialize(__deserializer)
                                                                                                                      {
                                                                                                                      _serde::export::Ok(__val)
                                                                                                                      =>
                                                                                                                      __val,
                                                                                                                      _serde::export::Err(__err)
                                                                                                                      =>
                                                                                                                      {
                                                                                                                          return _serde::export::Err(__err);
                                                                                                                      }
                                                                                                                  },
                                                                                                              phantom:
                                                                                                                  _serde::export::PhantomData,
                                                                                                              lifetime:
                                                                                                                  _serde::export::PhantomData,})
                                                                     }
                                                                 }
                                                                 match _serde::de::MapAccess::next_value::<__DeserializeWith<'de>>(&mut __map)
                                                                     {
                                                                     _serde::export::Ok(__val)
                                                                     =>
                                                                     __val,
                                                                     _serde::export::Err(__err)
                                                                     =>
                                                                     {
                                                                         return _serde::export::Err(__err);
                                                                     }
                                                                 }.value
                                                             });
                                }
                                __Field::__field4 => {
                                    if _serde::export::Option::is_some(&__field4)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("clean"));
                                    }
                                    __field4 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                            }
                        }
                        let __default: Self::Value =
                            _serde::export::Default::default();
                        let __field0 =
                            match __field0 {
                                _serde::export::Some(__field0) =>
                                __field0,
                                _serde::export::None =>
                                __default.parse_deps,
                            };
                        let __field1 =
                            match __field1 {
                                _serde::export::Some(__field1) =>
                                __field1,
                                _serde::export::None =>
                                __default.include,
                            };
                        let __field2 =
                            match __field2 {
                                _serde::export::Some(__field2) =>
                                __field2,
                                _serde::export::None =>
                                __default.exclude,
                            };
                        let __field3 =
                            match __field3 {
                                _serde::export::Some(__field3) =>
                                __field3,
                                _serde::export::None =>
                                __default.expand,
                            };
                        let __field4 =
                            match __field4 {
                                _serde::export::Some(__field4) =>
                                __field4,
                                _serde::export::None =>
                                __default.clean,
                            };
                        _serde::export::Ok(ParseConfig{parse_deps:
                                                           __field0,
                                                       include:
                                                           __field1,
                                                       exclude:
                                                           __field2,
                                                       expand:
                                                           __field3,
                                                       clean:
                                                           __field4,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["parse_deps", "include", "exclude", "expand",
                      "clean"];
                _serde::Deserializer::deserialize_struct(__deserializer,
                                                         "ParseConfig",
                                                         FIELDS,
                                                         __Visitor{marker:
                                                                       _serde::export::PhantomData::<ParseConfig>,
                                                                   lifetime:
                                                                       _serde::export::PhantomData,})
            }
        }
    };
#[allow(non_upper_case_globals,
        unused_attributes,
        unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_Config: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val ,
                         _serde :: export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err ) ; }
                         } });
        #[automatically_derived]
        impl <'de> _serde::Deserialize<'de> for Config where
         Config<>: _serde::export::Default {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __field10,
                    __field11,
                    __field12,
                    __field13,
                    __field14,
                    __field15,
                    __field16,
                    __field17,
                    __field18,
                    __field19,
                    __field20,
                    __field21,
                    __field22,
                }
                struct __FieldVisitor;
                impl <'de> _serde::de::Visitor<'de> for __FieldVisitor
                 {
                    type
                    Value
                    =
                    __Field;
                    fn expecting(&self,
                                 __formatter:
                                     &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "field identifier")
                    }
                    fn visit_u64<__E>(self, __value: u64)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            0u64 =>
                            _serde::export::Ok(__Field::__field0),
                            1u64 =>
                            _serde::export::Ok(__Field::__field1),
                            2u64 =>
                            _serde::export::Ok(__Field::__field2),
                            3u64 =>
                            _serde::export::Ok(__Field::__field3),
                            4u64 =>
                            _serde::export::Ok(__Field::__field4),
                            5u64 =>
                            _serde::export::Ok(__Field::__field5),
                            6u64 =>
                            _serde::export::Ok(__Field::__field6),
                            7u64 =>
                            _serde::export::Ok(__Field::__field7),
                            8u64 =>
                            _serde::export::Ok(__Field::__field8),
                            9u64 =>
                            _serde::export::Ok(__Field::__field9),
                            10u64 =>
                            _serde::export::Ok(__Field::__field10),
                            11u64 =>
                            _serde::export::Ok(__Field::__field11),
                            12u64 =>
                            _serde::export::Ok(__Field::__field12),
                            13u64 =>
                            _serde::export::Ok(__Field::__field13),
                            14u64 =>
                            _serde::export::Ok(__Field::__field14),
                            15u64 =>
                            _serde::export::Ok(__Field::__field15),
                            16u64 =>
                            _serde::export::Ok(__Field::__field16),
                            17u64 =>
                            _serde::export::Ok(__Field::__field17),
                            18u64 =>
                            _serde::export::Ok(__Field::__field18),
                            19u64 =>
                            _serde::export::Ok(__Field::__field19),
                            20u64 =>
                            _serde::export::Ok(__Field::__field20),
                            21u64 =>
                            _serde::export::Ok(__Field::__field21),
                            22u64 =>
                            _serde::export::Ok(__Field::__field22),
                            _ =>
                            _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                 &"field index 0 <= i < 23")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            "header" =>
                            _serde::export::Ok(__Field::__field0),
                            "includes" =>
                            _serde::export::Ok(__Field::__field1),
                            "sys_includes" =>
                            _serde::export::Ok(__Field::__field2),
                            "trailer" =>
                            _serde::export::Ok(__Field::__field3),
                            "include_guard" =>
                            _serde::export::Ok(__Field::__field4),
                            "no_includes" =>
                            _serde::export::Ok(__Field::__field5),
                            "autogen_warning" =>
                            _serde::export::Ok(__Field::__field6),
                            "include_version" =>
                            _serde::export::Ok(__Field::__field7),
                            "namespace" =>
                            _serde::export::Ok(__Field::__field8),
                            "namespaces" =>
                            _serde::export::Ok(__Field::__field9),
                            "braces" =>
                            _serde::export::Ok(__Field::__field10),
                            "line_length" =>
                            _serde::export::Ok(__Field::__field11),
                            "tab_width" =>
                            _serde::export::Ok(__Field::__field12),
                            "language" =>
                            _serde::export::Ok(__Field::__field13),
                            "style" =>
                            _serde::export::Ok(__Field::__field14),
                            "parse" =>
                            _serde::export::Ok(__Field::__field15),
                            "export" =>
                            _serde::export::Ok(__Field::__field16),
                            "fn" =>
                            _serde::export::Ok(__Field::__field17),
                            "struct" =>
                            _serde::export::Ok(__Field::__field18),
                            "enum" =>
                            _serde::export::Ok(__Field::__field19),
                            "const" =>
                            _serde::export::Ok(__Field::__field20),
                            "defines" =>
                            _serde::export::Ok(__Field::__field21),
                            "documentation" =>
                            _serde::export::Ok(__Field::__field22),
                            _ => {
                                _serde::export::Err(_serde::de::Error::unknown_field(__value,
                                                                                     FIELDS))
                            }
                        }
                    }
                    fn visit_bytes<__E>(self, __value: &[u8])
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            b"header" =>
                            _serde::export::Ok(__Field::__field0),
                            b"includes" =>
                            _serde::export::Ok(__Field::__field1),
                            b"sys_includes" =>
                            _serde::export::Ok(__Field::__field2),
                            b"trailer" =>
                            _serde::export::Ok(__Field::__field3),
                            b"include_guard" =>
                            _serde::export::Ok(__Field::__field4),
                            b"no_includes" =>
                            _serde::export::Ok(__Field::__field5),
                            b"autogen_warning" =>
                            _serde::export::Ok(__Field::__field6),
                            b"include_version" =>
                            _serde::export::Ok(__Field::__field7),
                            b"namespace" =>
                            _serde::export::Ok(__Field::__field8),
                            b"namespaces" =>
                            _serde::export::Ok(__Field::__field9),
                            b"braces" =>
                            _serde::export::Ok(__Field::__field10),
                            b"line_length" =>
                            _serde::export::Ok(__Field::__field11),
                            b"tab_width" =>
                            _serde::export::Ok(__Field::__field12),
                            b"language" =>
                            _serde::export::Ok(__Field::__field13),
                            b"style" =>
                            _serde::export::Ok(__Field::__field14),
                            b"parse" =>
                            _serde::export::Ok(__Field::__field15),
                            b"export" =>
                            _serde::export::Ok(__Field::__field16),
                            b"fn" =>
                            _serde::export::Ok(__Field::__field17),
                            b"struct" =>
                            _serde::export::Ok(__Field::__field18),
                            b"enum" =>
                            _serde::export::Ok(__Field::__field19),
                            b"const" =>
                            _serde::export::Ok(__Field::__field20),
                            b"defines" =>
                            _serde::export::Ok(__Field::__field21),
                            b"documentation" =>
                            _serde::export::Ok(__Field::__field22),
                            _ => {
                                let __value =
                                    &_serde::export::from_utf8_lossy(__value);
                                _serde::export::Err(_serde::de::Error::unknown_field(__value,
                                                                                     FIELDS))
                            }
                        }
                    }
                }
                impl <'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(__deserializer: __D)
                     -> _serde::export::Result<Self, __D::Error> where
                     __D: _serde::Deserializer<'de> {
                        _serde::Deserializer::deserialize_identifier(__deserializer,
                                                                     __FieldVisitor)
                    }
                }
                struct __Visitor<'de> where
                       Config<>: _serde::export::Default {
                    marker: _serde::export::PhantomData<Config>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de> _serde::de::Visitor<'de> for __Visitor<'de>
                 where Config<>: _serde::export::Default {
                    type
                    Value
                    =
                    Config;
                    fn expecting(&self,
                                 __formatter:
                                     &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "struct Config")
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A)
                     ->
                         _serde::export::Result<Self::Value,
                                                __A::Error> where
                     __A: _serde::de::SeqAccess<'de> {
                        let __default: Self::Value =
                            _serde::export::Default::default();
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Option<String>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"struct Config with 23 elements"));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<Vec<String>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                 &"struct Config with 23 elements"));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<Vec<String>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(2usize,
                                                                                                 &"struct Config with 23 elements"));
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<Option<String>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(3usize,
                                                                                                 &"struct Config with 23 elements"));
                                }
                            };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<Option<String>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(4usize,
                                                                                                 &"struct Config with 23 elements"));
                                }
                            };
                        let __field5 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(5usize,
                                                                                                 &"struct Config with 23 elements"));
                                }
                            };
                        let __field6 =
                            match match _serde::de::SeqAccess::next_element::<Option<String>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(6usize,
                                                                                                 &"struct Config with 23 elements"));
                                }
                            };
                        let __field7 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(7usize,
                                                                                                 &"struct Config with 23 elements"));
                                }
                            };
                        let __field8 =
                            match match _serde::de::SeqAccess::next_element::<Option<String>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(8usize,
                                                                                                 &"struct Config with 23 elements"));
                                }
                            };
                        let __field9 =
                            match match _serde::de::SeqAccess::next_element::<Option<Vec<String>>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(9usize,
                                                                                                 &"struct Config with 23 elements"));
                                }
                            };
                        let __field10 =
                            match match _serde::de::SeqAccess::next_element::<Braces>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(10usize,
                                                                                                 &"struct Config with 23 elements"));
                                }
                            };
                        let __field11 =
                            match match _serde::de::SeqAccess::next_element::<usize>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(11usize,
                                                                                                 &"struct Config with 23 elements"));
                                }
                            };
                        let __field12 =
                            match match _serde::de::SeqAccess::next_element::<usize>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(12usize,
                                                                                                 &"struct Config with 23 elements"));
                                }
                            };
                        let __field13 =
                            match match _serde::de::SeqAccess::next_element::<Language>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(13usize,
                                                                                                 &"struct Config with 23 elements"));
                                }
                            };
                        let __field14 =
                            match match _serde::de::SeqAccess::next_element::<Style>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(14usize,
                                                                                                 &"struct Config with 23 elements"));
                                }
                            };
                        let __field15 =
                            match match _serde::de::SeqAccess::next_element::<ParseConfig>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(15usize,
                                                                                                 &"struct Config with 23 elements"));
                                }
                            };
                        let __field16 =
                            match match _serde::de::SeqAccess::next_element::<ExportConfig>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(16usize,
                                                                                                 &"struct Config with 23 elements"));
                                }
                            };
                        let __field17 =
                            match match _serde::de::SeqAccess::next_element::<FunctionConfig>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(17usize,
                                                                                                 &"struct Config with 23 elements"));
                                }
                            };
                        let __field18 =
                            match match _serde::de::SeqAccess::next_element::<StructConfig>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(18usize,
                                                                                                 &"struct Config with 23 elements"));
                                }
                            };
                        let __field19 =
                            match match _serde::de::SeqAccess::next_element::<EnumConfig>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(19usize,
                                                                                                 &"struct Config with 23 elements"));
                                }
                            };
                        let __field20 =
                            match match _serde::de::SeqAccess::next_element::<ConstantConfig>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(20usize,
                                                                                                 &"struct Config with 23 elements"));
                                }
                            };
                        let __field21 =
                            match match _serde::de::SeqAccess::next_element::<HashMap<String,
                                                                                      String>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(21usize,
                                                                                                 &"struct Config with 23 elements"));
                                }
                            };
                        let __field22 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(22usize,
                                                                                                 &"struct Config with 23 elements"));
                                }
                            };
                        _serde::export::Ok(Config{header: __field0,
                                                  includes: __field1,
                                                  sys_includes:
                                                      __field2,
                                                  trailer: __field3,
                                                  include_guard:
                                                      __field4,
                                                  no_includes:
                                                      __field5,
                                                  autogen_warning:
                                                      __field6,
                                                  include_version:
                                                      __field7,
                                                  namespace: __field8,
                                                  namespaces:
                                                      __field9,
                                                  braces: __field10,
                                                  line_length:
                                                      __field11,
                                                  tab_width:
                                                      __field12,
                                                  language: __field13,
                                                  style: __field14,
                                                  parse: __field15,
                                                  export: __field16,
                                                  function: __field17,
                                                  structure:
                                                      __field18,
                                                  enumeration:
                                                      __field19,
                                                  constant: __field20,
                                                  defines: __field21,
                                                  documentation:
                                                      __field22,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     ->
                         _serde::export::Result<Self::Value,
                                                __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0:
                                _serde::export::Option<Option<String>> =
                            _serde::export::None;
                        let mut __field1:
                                _serde::export::Option<Vec<String>> =
                            _serde::export::None;
                        let mut __field2:
                                _serde::export::Option<Vec<String>> =
                            _serde::export::None;
                        let mut __field3:
                                _serde::export::Option<Option<String>> =
                            _serde::export::None;
                        let mut __field4:
                                _serde::export::Option<Option<String>> =
                            _serde::export::None;
                        let mut __field5:
                                _serde::export::Option<bool> =
                            _serde::export::None;
                        let mut __field6:
                                _serde::export::Option<Option<String>> =
                            _serde::export::None;
                        let mut __field7:
                                _serde::export::Option<bool> =
                            _serde::export::None;
                        let mut __field8:
                                _serde::export::Option<Option<String>> =
                            _serde::export::None;
                        let mut __field9:
                                _serde::export::Option<Option<Vec<String>>> =
                            _serde::export::None;
                        let mut __field10:
                                _serde::export::Option<Braces> =
                            _serde::export::None;
                        let mut __field11:
                                _serde::export::Option<usize> =
                            _serde::export::None;
                        let mut __field12:
                                _serde::export::Option<usize> =
                            _serde::export::None;
                        let mut __field13:
                                _serde::export::Option<Language> =
                            _serde::export::None;
                        let mut __field14:
                                _serde::export::Option<Style> =
                            _serde::export::None;
                        let mut __field15:
                                _serde::export::Option<ParseConfig> =
                            _serde::export::None;
                        let mut __field16:
                                _serde::export::Option<ExportConfig> =
                            _serde::export::None;
                        let mut __field17:
                                _serde::export::Option<FunctionConfig> =
                            _serde::export::None;
                        let mut __field18:
                                _serde::export::Option<StructConfig> =
                            _serde::export::None;
                        let mut __field19:
                                _serde::export::Option<EnumConfig> =
                            _serde::export::None;
                        let mut __field20:
                                _serde::export::Option<ConstantConfig> =
                            _serde::export::None;
                        let mut __field21:
                                _serde::export::Option<HashMap<String,
                                                               String>> =
                            _serde::export::None;
                        let mut __field22:
                                _serde::export::Option<bool> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                                  match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("header"));
                                    }
                                    __field0 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<String>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("includes"));
                                    }
                                    __field1 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Vec<String>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("sys_includes"));
                                    }
                                    __field2 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Vec<String>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("trailer"));
                                    }
                                    __field3 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<String>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field4 => {
                                    if _serde::export::Option::is_some(&__field4)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("include_guard"));
                                    }
                                    __field4 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<String>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field5 => {
                                    if _serde::export::Option::is_some(&__field5)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("no_includes"));
                                    }
                                    __field5 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field6 => {
                                    if _serde::export::Option::is_some(&__field6)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("autogen_warning"));
                                    }
                                    __field6 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<String>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field7 => {
                                    if _serde::export::Option::is_some(&__field7)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("include_version"));
                                    }
                                    __field7 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field8 => {
                                    if _serde::export::Option::is_some(&__field8)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("namespace"));
                                    }
                                    __field8 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<String>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field9 => {
                                    if _serde::export::Option::is_some(&__field9)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("namespaces"));
                                    }
                                    __field9 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<Vec<String>>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field10 => {
                                    if _serde::export::Option::is_some(&__field10)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("braces"));
                                    }
                                    __field10 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Braces>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field11 => {
                                    if _serde::export::Option::is_some(&__field11)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("line_length"));
                                    }
                                    __field11 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<usize>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field12 => {
                                    if _serde::export::Option::is_some(&__field12)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("tab_width"));
                                    }
                                    __field12 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<usize>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field13 => {
                                    if _serde::export::Option::is_some(&__field13)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("language"));
                                    }
                                    __field13 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Language>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field14 => {
                                    if _serde::export::Option::is_some(&__field14)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("style"));
                                    }
                                    __field14 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Style>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field15 => {
                                    if _serde::export::Option::is_some(&__field15)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("parse"));
                                    }
                                    __field15 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<ParseConfig>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field16 => {
                                    if _serde::export::Option::is_some(&__field16)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("export"));
                                    }
                                    __field16 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<ExportConfig>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field17 => {
                                    if _serde::export::Option::is_some(&__field17)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("fn"));
                                    }
                                    __field17 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<FunctionConfig>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field18 => {
                                    if _serde::export::Option::is_some(&__field18)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("struct"));
                                    }
                                    __field18 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<StructConfig>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field19 => {
                                    if _serde::export::Option::is_some(&__field19)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("enum"));
                                    }
                                    __field19 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<EnumConfig>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field20 => {
                                    if _serde::export::Option::is_some(&__field20)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("const"));
                                    }
                                    __field20 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<ConstantConfig>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field21 => {
                                    if _serde::export::Option::is_some(&__field21)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("defines"));
                                    }
                                    __field21 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<HashMap<String,
                                                                                                               String>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field22 => {
                                    if _serde::export::Option::is_some(&__field22)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("documentation"));
                                    }
                                    __field22 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                            }
                        }
                        let __default: Self::Value =
                            _serde::export::Default::default();
                        let __field0 =
                            match __field0 {
                                _serde::export::Some(__field0) =>
                                __field0,
                                _serde::export::None =>
                                __default.header,
                            };
                        let __field1 =
                            match __field1 {
                                _serde::export::Some(__field1) =>
                                __field1,
                                _serde::export::None =>
                                __default.includes,
                            };
                        let __field2 =
                            match __field2 {
                                _serde::export::Some(__field2) =>
                                __field2,
                                _serde::export::None =>
                                __default.sys_includes,
                            };
                        let __field3 =
                            match __field3 {
                                _serde::export::Some(__field3) =>
                                __field3,
                                _serde::export::None =>
                                __default.trailer,
                            };
                        let __field4 =
                            match __field4 {
                                _serde::export::Some(__field4) =>
                                __field4,
                                _serde::export::None =>
                                __default.include_guard,
                            };
                        let __field5 =
                            match __field5 {
                                _serde::export::Some(__field5) =>
                                __field5,
                                _serde::export::None =>
                                __default.no_includes,
                            };
                        let __field6 =
                            match __field6 {
                                _serde::export::Some(__field6) =>
                                __field6,
                                _serde::export::None =>
                                __default.autogen_warning,
                            };
                        let __field7 =
                            match __field7 {
                                _serde::export::Some(__field7) =>
                                __field7,
                                _serde::export::None =>
                                __default.include_version,
                            };
                        let __field8 =
                            match __field8 {
                                _serde::export::Some(__field8) =>
                                __field8,
                                _serde::export::None =>
                                __default.namespace,
                            };
                        let __field9 =
                            match __field9 {
                                _serde::export::Some(__field9) =>
                                __field9,
                                _serde::export::None =>
                                __default.namespaces,
                            };
                        let __field10 =
                            match __field10 {
                                _serde::export::Some(__field10) =>
                                __field10,
                                _serde::export::None =>
                                __default.braces,
                            };
                        let __field11 =
                            match __field11 {
                                _serde::export::Some(__field11) =>
                                __field11,
                                _serde::export::None =>
                                __default.line_length,
                            };
                        let __field12 =
                            match __field12 {
                                _serde::export::Some(__field12) =>
                                __field12,
                                _serde::export::None =>
                                __default.tab_width,
                            };
                        let __field13 =
                            match __field13 {
                                _serde::export::Some(__field13) =>
                                __field13,
                                _serde::export::None =>
                                __default.language,
                            };
                        let __field14 =
                            match __field14 {
                                _serde::export::Some(__field14) =>
                                __field14,
                                _serde::export::None =>
                                __default.style,
                            };
                        let __field15 =
                            match __field15 {
                                _serde::export::Some(__field15) =>
                                __field15,
                                _serde::export::None =>
                                __default.parse,
                            };
                        let __field16 =
                            match __field16 {
                                _serde::export::Some(__field16) =>
                                __field16,
                                _serde::export::None =>
                                __default.export,
                            };
                        let __field17 =
                            match __field17 {
                                _serde::export::Some(__field17) =>
                                __field17,
                                _serde::export::None =>
                                __default.function,
                            };
                        let __field18 =
                            match __field18 {
                                _serde::export::Some(__field18) =>
                                __field18,
                                _serde::export::None =>
                                __default.structure,
                            };
                        let __field19 =
                            match __field19 {
                                _serde::export::Some(__field19) =>
                                __field19,
                                _serde::export::None =>
                                __default.enumeration,
                            };
                        let __field20 =
                            match __field20 {
                                _serde::export::Some(__field20) =>
                                __field20,
                                _serde::export::None =>
                                __default.constant,
                            };
                        let __field21 =
                            match __field21 {
                                _serde::export::Some(__field21) =>
                                __field21,
                                _serde::export::None =>
                                __default.defines,
                            };
                        let __field22 =
                            match __field22 {
                                _serde::export::Some(__field22) =>
                                __field22,
                                _serde::export::None =>
                                __default.documentation,
                            };
                        _serde::export::Ok(Config{header: __field0,
                                                  includes: __field1,
                                                  sys_includes:
                                                      __field2,
                                                  trailer: __field3,
                                                  include_guard:
                                                      __field4,
                                                  no_includes:
                                                      __field5,
                                                  autogen_warning:
                                                      __field6,
                                                  include_version:
                                                      __field7,
                                                  namespace: __field8,
                                                  namespaces:
                                                      __field9,
                                                  braces: __field10,
                                                  line_length:
                                                      __field11,
                                                  tab_width:
                                                      __field12,
                                                  language: __field13,
                                                  style: __field14,
                                                  parse: __field15,
                                                  export: __field16,
                                                  function: __field17,
                                                  structure:
                                                      __field18,
                                                  enumeration:
                                                      __field19,
                                                  constant: __field20,
                                                  defines: __field21,
                                                  documentation:
                                                      __field22,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["header", "includes", "sys_includes", "trailer",
                      "include_guard", "no_includes",
                      "autogen_warning", "include_version",
                      "namespace", "namespaces", "braces",
                      "line_length", "tab_width", "language", "style",
                      "parse", "export", "fn", "struct", "enum",
                      "const", "defines", "documentation"];
                _serde::Deserializer::deserialize_struct(__deserializer,
                                                         "Config",
                                                         FIELDS,
                                                         __Visitor{marker:
                                                                       _serde::export::PhantomData::<Config>,
                                                                   lifetime:
                                                                       _serde::export::PhantomData,})
            }
        }
    };
