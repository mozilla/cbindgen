/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::str::FromStr;

use bindgen::ir::{Enum, Item};

/// The type of identifier to be renamed.
#[derive(Debug, Clone, Copy)]
pub enum IdentifierType<'a> {
    StructMember,
    EnumVariant(&'a Enum),
    FunctionArg,
    Enum,
}

impl<'a> IdentifierType<'a> {
    fn to_str(&'a self) -> &'static str {
        match *self {
            IdentifierType::StructMember => "m",
            IdentifierType::EnumVariant(..) => "",
            IdentifierType::FunctionArg => "a",
            IdentifierType::Enum => "",
        }
    }
}

/// A rule to apply to an identifier when generating bindings.
#[derive(Debug, Clone, Copy)]
pub enum RenameRule {
    /// Do not apply any renaming. The default.
    None,
    /// Converts the identifier to PascalCase and adds a context dependent prefix
    GeckoCase,
    /// Converts the identifier to lower case.
    LowerCase,
    /// Converts the identifier to upper case.
    UpperCase,
    /// Converts the identifier to PascalCase.
    PascalCase,
    /// Converts the identifier to camelCase.
    CamelCase,
    /// Converts the identifier to snake_case.
    SnakeCase,
    /// Converts the identifier to SCREAMING_SNAKE_CASE.
    ScreamingSnakeCase,
    /// Converts the identifier to SCREAMING_SNAKE_CASE and prefixes enum variants
    /// with the enum name.
    QualifiedScreamingSnakeCase,
}

impl RenameRule {
    /// Applies the rename rule to a string that is formatted in PascalCase.
    pub fn apply_to_pascal_case(self, text: &str, context: IdentifierType) -> String {
        if text.is_empty() {
            return String::new();
        }

        match self {
            Self::None => String::from(text),
            Self::GeckoCase => context.to_str().to_owned() + text,
            Self::LowerCase => text.to_lowercase(),
            Self::UpperCase => text.to_uppercase(),
            Self::PascalCase => text.to_owned(),
            Self::CamelCase => text[..1].to_lowercase() + &text[1..],
            Self::SnakeCase => {
                // Do not add additional `_` if the string already contains `_` e.g. `__Field`
                // Do not split consecutive capital letters
                let mut result = String::new();
                let mut add_separator = true;
                let mut prev_uppercase = false;
                for (i, c) in text.char_indices() {
                    if c == '_' {
                        add_separator = false;
                        prev_uppercase = false;
                    }
                    if c.is_uppercase() {
                        if i != 0 && add_separator && !prev_uppercase {
                            result.push_str("_");
                        } else {
                            add_separator = true;
                        }
                        prev_uppercase = true;
                    } else {
                        prev_uppercase = false;
                    }
                    for x in c.to_lowercase() {
                        result.push(x);
                    }
                }
                result
            }
            Self::ScreamingSnakeCase => {
                // Same as SnakeCase code above, but uses to_uppercase
                let mut result = String::new();
                let mut add_separator = true;
                let mut prev_uppercase = false;
                for (i, c) in text.char_indices() {
                    if c == '_' {
                        add_separator = false;
                        prev_uppercase = false;
                    }
                    if c.is_uppercase() {
                        if i != 0 && add_separator && !prev_uppercase {
                            result.push_str("_");
                        } else {
                            add_separator = true;
                        }
                        prev_uppercase = true;
                    } else {
                        prev_uppercase = false;
                    }
                    for x in c.to_uppercase() {
                        result.push(x);
                    }
                }
                result
            }
            Self::QualifiedScreamingSnakeCase => {
                let mut result = String::new();

                if let IdentifierType::EnumVariant(e) = context {
                    if let RenameRule::QualifiedScreamingSnakeCase = self {
                        result.push_str(
                            &RenameRule::ScreamingSnakeCase
                                .apply_to_pascal_case(e.path().name(), IdentifierType::Enum),
                        );
                        result.push_str("_");
                    }
                }

                result
                    .push_str(&RenameRule::ScreamingSnakeCase.apply_to_pascal_case(&text, context));
                result
            }
        }
    }

    /// Applies the rename rule to a string that is formatted in snake_case.
    pub fn apply_to_snake_case(self, mut text: &str, context: IdentifierType) -> String {
        if text.is_empty() {
            return String::new();
        }

        match self {
            Self::None => String::from(text),
            Self::GeckoCase => {
                if &text[..1] == "_" {
                    text = &text[1..];
                }

                context.to_str().to_owned()
                    + &RenameRule::PascalCase.apply_to_snake_case(text, context)
            }
            Self::LowerCase => text.to_lowercase(),
            Self::UpperCase => text.to_uppercase(),
            Self::PascalCase => {
                let mut result = String::new();
                let mut is_uppercase = true;
                for c in text.chars() {
                    if c == '_' {
                        is_uppercase = true;
                        continue;
                    }

                    if is_uppercase {
                        for x in c.to_uppercase() {
                            result.push(x);
                        }
                        is_uppercase = false;
                    } else {
                        result.push(c);
                    }
                }
                result
            }
            Self::CamelCase => {
                // Same as PascalCase code above, but is_uppercase = false to start
                let mut result = String::new();
                let mut is_uppercase = false;
                for c in text.chars() {
                    if c == '_' {
                        is_uppercase = true;
                        continue;
                    }

                    if is_uppercase {
                        for x in c.to_uppercase() {
                            result.push(x);
                        }
                        is_uppercase = false;
                    } else {
                        result.push(c);
                    }
                }
                result
            }
            Self::SnakeCase => text.to_owned(),
            Self::ScreamingSnakeCase => text.to_owned().to_uppercase(),
            Self::QualifiedScreamingSnakeCase => {
                let mut result = String::new();

                if let IdentifierType::EnumVariant(e) = context {
                    if let Self::QualifiedScreamingSnakeCase = self {
                        result.push_str(
                            &Self::ScreamingSnakeCase
                                .apply_to_snake_case(e.path().name(), IdentifierType::Enum),
                        );
                        result.push_str("_");
                    }
                }

                result
                    .push_str(&Self::ScreamingSnakeCase.apply_to_snake_case(&text, context));
                result
            }
        }
    }
}

impl Default for RenameRule {
    fn default() -> RenameRule {
        RenameRule::None
    }
}

impl FromStr for RenameRule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "None" => Ok(Self::None),

            "mGeckoCase" => Ok(Self::GeckoCase),
            "GeckoCase" => Ok(Self::GeckoCase),
            "gecko_case" => Ok(Self::GeckoCase),

            "lowercase" => Ok(Self::LowerCase),
            "LowerCase" => Ok(Self::LowerCase),
            "lower_case" => Ok(Self::LowerCase),

            "UPPERCASE" => Ok(Self::UpperCase),
            "UpperCase" => Ok(Self::UpperCase),
            "upper_case" => Ok(Self::UpperCase),

            "PascalCase" => Ok(Self::PascalCase),
            "pascal_case" => Ok(Self::PascalCase),

            "camelCase" => Ok(Self::CamelCase),
            "CamelCase" => Ok(Self::CamelCase),
            "camel_case" => Ok(Self::CamelCase),

            "snake_case" => Ok(Self::SnakeCase),
            "SnakeCase" => Ok(Self::SnakeCase),

            "SCREAMING_SNAKE_CASE" => Ok(Self::ScreamingSnakeCase),
            "ScreamingSnakeCase" => Ok(Self::ScreamingSnakeCase),
            "screaming_snake_case" => Ok(Self::ScreamingSnakeCase),

            "QUALIFIED_SCREAMING_SNAKE_CASE" => Ok(Self::QualifiedScreamingSnakeCase),
            "QualifiedScreamingSnakeCase" => Ok(Self::QualifiedScreamingSnakeCase),
            "qualified_screaming_snake_case" => Ok(Self::QualifiedScreamingSnakeCase),

            _ => Err(format!("Unrecognized RenameRule: '{}'.", s)),
        }
    }
}

deserialize_enum_str!(RenameRule);
