/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::str::FromStr;

use crate::bindgen::ir::{Enum, Item};

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
    /// Applies the rename rule to a string
    pub fn apply(self, text: &str, context: IdentifierType) -> String {
        use heck::*;

        if text.is_empty() {
            return String::new();
        }

        match self {
            RenameRule::None => text.to_owned(),
            RenameRule::GeckoCase => context.to_str().to_owned() + &text.to_camel_case(),
            RenameRule::LowerCase => text.to_lowercase(),
            RenameRule::UpperCase => text.to_uppercase(),
            RenameRule::PascalCase => text.to_camel_case(),
            RenameRule::CamelCase => text.to_mixed_case(),
            RenameRule::SnakeCase => text.to_snake_case(),
            RenameRule::ScreamingSnakeCase => text.to_shouty_snake_case(),
            RenameRule::QualifiedScreamingSnakeCase => {
                let mut result = String::new();

                if let IdentifierType::EnumVariant(e) = context {
                    result.push_str(
                        &RenameRule::ScreamingSnakeCase
                            .apply(e.path().name(), IdentifierType::Enum),
                    );
                    result.push_str("_");
                }

                result.push_str(&RenameRule::ScreamingSnakeCase.apply(&text, context));
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

    fn from_str(s: &str) -> Result<RenameRule, Self::Err> {
        match s {
            "none" => Ok(RenameRule::None),
            "None" => Ok(RenameRule::None),

            "mGeckoCase" => Ok(RenameRule::GeckoCase),
            "GeckoCase" => Ok(RenameRule::GeckoCase),
            "gecko_case" => Ok(RenameRule::GeckoCase),

            "lowercase" => Ok(RenameRule::LowerCase),
            "LowerCase" => Ok(RenameRule::LowerCase),
            "lower_case" => Ok(RenameRule::LowerCase),

            "UPPERCASE" => Ok(RenameRule::UpperCase),
            "UpperCase" => Ok(RenameRule::UpperCase),
            "upper_case" => Ok(RenameRule::UpperCase),

            "PascalCase" => Ok(RenameRule::PascalCase),
            "pascal_case" => Ok(RenameRule::PascalCase),

            "camelCase" => Ok(RenameRule::CamelCase),
            "CamelCase" => Ok(RenameRule::CamelCase),
            "camel_case" => Ok(RenameRule::CamelCase),

            "snake_case" => Ok(RenameRule::SnakeCase),
            "SnakeCase" => Ok(RenameRule::SnakeCase),

            "SCREAMING_SNAKE_CASE" => Ok(RenameRule::ScreamingSnakeCase),
            "ScreamingSnakeCase" => Ok(RenameRule::ScreamingSnakeCase),
            "screaming_snake_case" => Ok(RenameRule::ScreamingSnakeCase),

            "QUALIFIED_SCREAMING_SNAKE_CASE" => Ok(RenameRule::QualifiedScreamingSnakeCase),
            "QualifiedScreamingSnakeCase" => Ok(RenameRule::QualifiedScreamingSnakeCase),
            "qualified_screaming_snake_case" => Ok(RenameRule::QualifiedScreamingSnakeCase),

            _ => Err(format!("Unrecognized RenameRule: '{}'.", s)),
        }
    }
}

deserialize_enum_str!(RenameRule);
