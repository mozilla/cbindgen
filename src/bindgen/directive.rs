/// A simple system for specifying directives to the bindings
/// generator. Needs some more thought to be scalable.
/// Examples:
/// * cbindgen:field-names=[mHandle, mNamespace]
/// * cbindgen:function-postfix=WR_DESTRUCTOR_SAFE
#[derive(Debug, Clone)]
pub enum Directive {
    SetFieldNames(Vec<String>),
    SetFunctionPrefix(String),
    SetFunctionPostfix(String),
    SetStructGenOpEq(bool),
    SetStructGenOpNeq(bool),
    SetStructGenOpLt(bool),
    SetStructGenOpLte(bool),
    SetStructGenOpGt(bool),
    SetStructGenOpGte(bool),
}

pub type DirectiveParseResult<T> = Result<T, String>;

impl Directive {
    pub fn parse(text: String) -> DirectiveParseResult<Vec<Directive>> {
        let mut directives = Vec::new();

        for line in text.lines().map(|x| x.trim_left_matches("///").trim()) {
            if !line.starts_with("cbindgen:") {
                continue;
            }
            let directive = &line[9..];
            let parts: Vec<&str> = directive.split("=")
                                            .map(|x| x.trim())
                                            .collect();

            if parts.len() != 2 {
                return Err(format!("couldn't parse {}", line));
            }

            match parts[0] {
                "field-names" => {
                    let field_list = parts[1].trim_left_matches("[")
                                             .trim_right_matches("]");
                    let fields = field_list.split(',')
                                           .map(|x| x.trim().to_string())
                                           .collect();

                    directives.push(Directive::SetFieldNames(fields));
                }
                "function-prefix" => {
                    directives.push(Directive::SetFunctionPrefix(parts[1].to_string()));
                }
                "function-postfix" => {
                    directives.push(Directive::SetFunctionPostfix(parts[1].to_string()));
                }
                "struct-gen-op-eq" => {
                    let value = match parts[1].parse::<bool>() {
                        Ok(x) => x,
                        Err(_) => return Err(format!("couldn't parse {}", line)),
                    };
                    directives.push(Directive::SetStructGenOpEq(value));
                }
                "struct-gen-op-neq" => {
                    let value = match parts[1].parse::<bool>() {
                        Ok(x) => x,
                        Err(_) => return Err(format!("couldn't parse {}", line)),
                    };
                    directives.push(Directive::SetStructGenOpNeq(value));
                }
                "struct-gen-op-lt" => {
                    let value = match parts[1].parse::<bool>() {
                        Ok(x) => x,
                        Err(_) => return Err(format!("couldn't parse {}", line)),
                    };
                    directives.push(Directive::SetStructGenOpLt(value));
                }
                "struct-gen-op-lte" => {
                    let value = match parts[1].parse::<bool>() {
                        Ok(x) => x,
                        Err(_) => return Err(format!("couldn't parse {}", line)),
                    };
                    directives.push(Directive::SetStructGenOpLte(value));
                }
                "struct-gen-op-gt" => {
                    let value = match parts[1].parse::<bool>() {
                        Ok(x) => x,
                        Err(_) => return Err(format!("couldn't parse {}", line)),
                    };
                    directives.push(Directive::SetStructGenOpGt(value));
                }
                "struct-gen-op-gte" => {
                    let value = match parts[1].parse::<bool>() {
                        Ok(x) => x,
                        Err(_) => return Err(format!("couldn't parse {}", line)),
                    };
                    directives.push(Directive::SetStructGenOpGte(value));
                }
                _ => return Err(format!("couldn't parse {}", line)),
            }
        }

        Ok(directives)
    }
}

pub trait DirectiveHelpers {
    fn set_field_names(&self) -> Option<Vec<String>>;
    fn set_function_prefix(&self) -> Option<String>;
    fn set_function_postfix(&self) -> Option<String>;
    fn set_struct_gen_op_eq(&self) -> Option<bool>;
    fn set_struct_gen_op_neq(&self) -> Option<bool>;
    fn set_struct_gen_op_lt(&self) -> Option<bool>;
    fn set_struct_gen_op_lte(&self) -> Option<bool>;
    fn set_struct_gen_op_gt(&self) -> Option<bool>;
    fn set_struct_gen_op_gte(&self) -> Option<bool>;
}
impl DirectiveHelpers for Vec<Directive> {
    fn set_field_names(&self) -> Option<Vec<String>> {
        for directive in self {
            if let &Directive::SetFieldNames(ref x) = directive {
                return Some(x.clone());
            }
        }
        None
    }

    fn set_function_prefix(&self) -> Option<String> {
        for directive in self {
            if let &Directive::SetFunctionPrefix(ref x) = directive {
                return Some(x.clone());
            }
        }
        None
    }
    fn set_function_postfix(&self) -> Option<String> {
        for directive in self {
            if let &Directive::SetFunctionPostfix(ref x) = directive {
                return Some(x.clone());
            }
        }
        None
    }

    fn set_struct_gen_op_eq(&self) -> Option<bool> {
        for directive in self {
            if let &Directive::SetStructGenOpEq(ref x) = directive {
                return Some(*x);
            }
        }
        None
    }
    fn set_struct_gen_op_neq(&self) -> Option<bool> {
        for directive in self {
            if let &Directive::SetStructGenOpNeq(ref x) = directive {
                return Some(*x);
            }
        }
        None
    }
    fn set_struct_gen_op_lt(&self) -> Option<bool> {
        for directive in self {
            if let &Directive::SetStructGenOpLt(ref x) = directive {
                return Some(*x);
            }
        }
        None
    }
    fn set_struct_gen_op_lte(&self) -> Option<bool> {
        for directive in self {
            if let &Directive::SetStructGenOpLte(ref x) = directive {
                return Some(*x);
            }
        }
        None
    }
    fn set_struct_gen_op_gt(&self) -> Option<bool> {
        for directive in self {
            if let &Directive::SetStructGenOpGt(ref x) = directive {
                return Some(*x);
            }
        }
        None
    }
    fn set_struct_gen_op_gte(&self) -> Option<bool> {
        for directive in self {
            if let &Directive::SetStructGenOpGte(ref x) = directive {
                return Some(*x);
            }
        }
        None
    }
}
