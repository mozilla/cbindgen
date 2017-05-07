use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum RenameContext {
    StructMember,
    EnumVariant,
    FunctionArg,
}
impl RenameContext {
    fn to_str(&self) -> &'static str {
        match *self {
            RenameContext::StructMember => "m",
            RenameContext::EnumVariant => "",
            RenameContext::FunctionArg => "a",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RenameRule {
    GeckoCase,
    LowerCase,
    UpperCase,
    PascalCase,
    CamelCase,
    SnakeCase,
    ScreamingSnakeCase,
}

impl RenameRule {
    pub fn apply_to_pascal_case(&self, text: &str, context: RenameContext) -> String {
        if text.len() == 0 {
            return String::new();
        }

        match *self {
            RenameRule::GeckoCase => context.to_str().to_owned() + text,
            RenameRule::LowerCase => text.to_lowercase(),
            RenameRule::UpperCase => text.to_uppercase(),
            RenameRule::PascalCase => text.to_owned(),
            RenameRule::CamelCase => {
                text[..1].to_lowercase() + &text[1..]
            }
            RenameRule::SnakeCase => {
                let mut result = String::new();
                for (i, c) in text.char_indices() {
                    if c.is_uppercase() && i != 0 {
                        result.push_str("_");
                    }
                    for x in c.to_lowercase() {
                        result.push(x);
                    }
                }
                result
            }
            RenameRule::ScreamingSnakeCase => {
                // Same as SnakeCase code above, but uses to_uppercase
                let mut result = String::new();
                for (i, c) in text.char_indices() {
                    if c.is_uppercase() && i != 0 {
                        result.push_str("_");
                    }
                    for x in c.to_uppercase() {
                        result.push(x);
                    }
                }
                result
            }
        }
    }

    pub fn apply_to_snake_case(&self, text: &str, context: RenameContext) -> String {
        if text.len() == 0 {
            return String::new();
        }

        match *self {
            RenameRule::GeckoCase => {
                context.to_str().to_owned() +
                    &RenameRule::PascalCase.apply_to_snake_case(text, context)
            }
            RenameRule::LowerCase => text.to_lowercase(),
            RenameRule::UpperCase => text.to_uppercase(),
            RenameRule::PascalCase => {
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
            RenameRule::CamelCase => {
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
            RenameRule::SnakeCase => text.to_owned(),
            RenameRule::ScreamingSnakeCase => text.to_owned().to_uppercase(),
        }
    }
}

impl FromStr for RenameRule {
    type Err = String;

    fn from_str(s: &str) -> Result<RenameRule, Self::Err> {
        match s {
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

            _ => Err(format!("unrecognized RenameRule: '{}'", s)),
        }
    }
}
deserialize_enum_str!(RenameRule);
