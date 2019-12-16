/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;
use std::default::Default;
use std::str::FromStr;
use std::{fmt, fs, path::Path as StdPath};

use serde::de::value::{MapAccessDeserializer, SeqAccessDeserializer};
use serde::de::{Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};

use toml;

use bindgen::ir::annotation::AnnotationSet;
use bindgen::ir::path::Path;
use bindgen::ir::repr::ReprAlign;
pub use bindgen::rename::RenameRule;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// A language type to generate bindings for.
#[derive(Debug, Copy, Clone, PartialEq)]
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

/// How the comments containing documentation should be styled.
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum DocumentationStyle {
    C,
    C99,
    Doxy,
    Cxx,
    Auto,
}

impl FromStr for DocumentationStyle {
    type Err = String;

    fn from_str(s: &str) -> Result<DocumentationStyle, Self::Err> {
        match s.to_lowercase().as_ref() {
            "c" => Ok(DocumentationStyle::C),
            "c99" => Ok(DocumentationStyle::C99),
            "cxx" => Ok(DocumentationStyle::Cxx),
            "c++" => Ok(DocumentationStyle::Cxx),
            "doxy" => Ok(DocumentationStyle::Doxy),
            "auto" => Ok(DocumentationStyle::Auto),
            _ => Err(format!("Unrecognized documentation style: '{}'.", s)),
        }
    }
}

deserialize_enum_str!(DocumentationStyle);

/// A style of Style to use when generating structs and enums.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Style {
    Both,
    Tag,
    Type,
}

impl Style {
    pub fn generate_tag(self) -> bool {
        match self {
            Style::Both | Style::Tag => true,
            Style::Type => false,
        }
    }

    pub fn generate_typedef(self) -> bool {
        match self {
            Style::Both | Style::Type => true,
            Style::Tag => false,
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
#[derive(Debug, Clone, Deserialize, Default)]
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
    /// Table of raw strings to append to the body of items.
    pub body: HashMap<String, String>,
    /// A prefix to add before the name of every item
    pub prefix: Option<String>,
    /// Types of items to generate.
    pub item_types: Vec<ItemType>,
    /// Whether renaming overrides or extends prefixing.
    pub renaming_overrides_prefixing: bool,
}

impl ExportConfig {
    pub(crate) fn should_generate(&self, item_type: ItemType) -> bool {
        self.item_types.is_empty() || self.item_types.contains(&item_type)
    }

    pub(crate) fn extra_body(&self, path: &Path) -> Option<&str> {
        self.body.get(path.name()).map(|s| s.trim_matches('\n'))
    }

    pub(crate) fn rename(&self, item_name: &mut String) {
        if let Some(name) = self.rename.get(item_name) {
            *item_name = name.clone();
            if self.renaming_overrides_prefixing {
                return;
            }
        }
        if let Some(ref prefix) = self.prefix {
            item_name.insert_str(0, &prefix);
        }
    }
}

/// Settings to apply to generated types with layout modifiers.
#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct LayoutConfig {
    /// The way to annotate C types as #[repr(packed)].
    pub packed: Option<String>,
    /// The way to annotate C types as #[repr(align(...))]. This is assumed to be a functional
    /// macro which takes a single argument (the alignment).
    pub aligned_n: Option<String>,
}

impl LayoutConfig {
    pub(crate) fn ensure_safe_to_represent(&self, align: &ReprAlign) -> Result<(), String> {
        match (align, &self.packed, &self.aligned_n) {
            (ReprAlign::Packed, None, _) => Err("Cannot safely represent #[repr(packed)] type without configured 'packed' annotation.".to_string()),
            (ReprAlign::Align(_), _, None) => Err("Cannot safely represent #[repr(aligned(...))] type without configured 'aligned_n' annotation.".to_string()),
            _ => Ok(()),
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
    /// The way to annotation this function as #[must_use].
    pub must_use: Option<String>,
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
            must_use: None,
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
#[derive(Debug, Default, Clone, Deserialize)]
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
    /// Whether associated constants should be in the body. Only applicable to
    /// non-transparent structs, and in C++-only.
    pub associated_constants_in_body: bool,
    /// The way to annotate this struct as #[must_use].
    pub must_use: Option<String>,
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
#[derive(Debug, Clone, Default, Deserialize)]
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
    /// Whether to generate `AsX() const` methods for tagged enums.
    pub derive_const_casts: bool,
    /// Whether to generate `AsX()` methods for tagged enums.
    pub derive_mut_casts: bool,
    /// The name of the macro to use for `derive_{const,mut}casts`. If custom, you're
    /// responsible to provide the necessary header, otherwise `assert` will be
    /// used, and `<cassert>` will be included.
    pub cast_assert_name: Option<String>,
    /// The way to annotation this enum as #[must_use].
    pub must_use: Option<String>,
    /// Whether to generate destructors of tagged enums.
    pub derive_tagged_enum_destructor: bool,
    /// Whether to generate copy-constructors of tagged enums.
    pub derive_tagged_enum_copy_constructor: bool,
    /// Whether to generate copy-assignment operators of tagged enums.
    ///
    /// This is only generated if a copy constructor for the same tagged enum is
    /// generated as well.
    pub derive_tagged_enum_copy_assignment: bool,
    /// Whether to generate empty, private default-constructors for tagged
    /// enums.
    pub private_default_tagged_enum_constructor: bool,
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
    pub(crate) fn derive_const_casts(&self, annotations: &AnnotationSet) -> bool {
        if let Some(x) = annotations.bool("derive-const-casts") {
            return x;
        }
        self.derive_const_casts
    }
    pub(crate) fn derive_mut_casts(&self, annotations: &AnnotationSet) -> bool {
        if let Some(x) = annotations.bool("derive-mut-casts") {
            return x;
        }
        self.derive_mut_casts
    }
    pub(crate) fn derive_tagged_enum_destructor(&self, annotations: &AnnotationSet) -> bool {
        if let Some(x) = annotations.bool("derive-tagged-enum-destructor") {
            return x;
        }
        self.derive_tagged_enum_destructor
    }
    pub(crate) fn derive_tagged_enum_copy_constructor(&self, annotations: &AnnotationSet) -> bool {
        if let Some(x) = annotations.bool("derive-tagged-enum-copy-constructor") {
            return x;
        }
        self.derive_tagged_enum_copy_constructor
    }
    pub(crate) fn derive_tagged_enum_copy_assignment(&self, annotations: &AnnotationSet) -> bool {
        if let Some(x) = annotations.bool("derive-tagged-enum-copy-assignment") {
            return x;
        }
        self.derive_tagged_enum_copy_assignment
    }
    pub(crate) fn private_default_tagged_enum_constructor(
        &self,
        annotations: &AnnotationSet,
    ) -> bool {
        if let Some(x) = annotations.bool("private-default-tagged-enum-constructor") {
            return x;
        }
        self.private_default_tagged_enum_constructor
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

/// Settings for custom macro expansion.
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct MacroExpansionConfig {
    /// Whether the `bitflags` macro should be expanded.
    pub bitflags: bool,
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
#[derive(Debug, Default, Clone, Deserialize)]
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
    /// List of crate names which generate consts, statics, and fns. By default
    /// no dependent crates generate them.
    pub extra_bindings: Vec<String>,
}

impl ParseConfig {
    pub(crate) fn should_generate_top_level_item(
        &self,
        crate_name: &str,
        binding_crate_name: &str,
    ) -> bool {
        if crate_name == binding_crate_name {
            // Always generate items for the binding crate.
            return true;
        }

        self.extra_bindings.iter().any(|dep| dep == crate_name)
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
    /// An optional list of namespaces to declare as using. Only applicable when language="C++"
    pub using_namespaces: Option<Vec<String>>,
    /// The style to use for braces
    pub braces: Braces,
    /// The preferred length of a line, used for auto breaking function arguments
    pub line_length: usize,
    /// The amount of spaces in a tab
    pub tab_width: usize,
    /// The language to output bindings for
    pub language: Language,
    /// Include preprocessor defines in C bindings to ensure C++ compatibility
    pub cpp_compat: bool,
    /// The style to declare structs, enums and unions in for C
    pub style: Style,
    /// The configuration options for parsing
    pub parse: ParseConfig,
    /// The configuration options for exporting
    pub export: ExportConfig,
    /// The configuration options for macros.
    pub macro_expansion: MacroExpansionConfig,
    /// The configuration options for type layouts.
    pub layout: LayoutConfig,
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
    /// How documentation comments should be styled.
    pub documentation_style: DocumentationStyle,
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
            using_namespaces: None,
            braces: Braces::SameLine,
            line_length: 100,
            tab_width: 2,
            language: Language::Cxx,
            cpp_compat: false,
            style: Style::Type,
            macro_expansion: Default::default(),
            parse: ParseConfig::default(),
            export: ExportConfig::default(),
            layout: LayoutConfig::default(),
            function: FunctionConfig::default(),
            structure: StructConfig::default(),
            enumeration: EnumConfig::default(),
            constant: ConstantConfig::default(),
            defines: HashMap::new(),
            documentation: true,
            documentation_style: DocumentationStyle::Auto,
        }
    }
}

impl Config {
    pub fn from_file<P: AsRef<StdPath>>(file_name: P) -> Result<Config, String> {
        let config_text = fs::read_to_string(file_name.as_ref()).or_else(|_| {
            Err(format!(
                "Couldn't open config file: {}.",
                file_name.as_ref().display()
            ))
        })?;

        match toml::from_str::<Config>(&config_text) {
            Ok(x) => Ok(x),
            Err(e) => Err(format!("Couldn't parse config file: {}.", e)),
        }
    }

    pub fn from_root_or_default<P: AsRef<StdPath>>(root: P) -> Config {
        let c = root.as_ref().join("cbindgen.toml");

        if c.exists() {
            Config::from_file(c).unwrap()
        } else {
            Config::default()
        }
    }
}
