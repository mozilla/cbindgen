use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum DirectiveValue {
    List(Vec<String>),
    Atom(Option<String>),
    Bool(bool),
}

/// A simple system for specifying properties on items
///
/// a directive is given by cbindgen:PROPERTY=VALUE
/// where PROPERTY depends on the item
/// where VALUE can be
///  * list - [item1, item2, item3]
///  * atom - foo
///  * bool - true,false
/// Examples:
/// * cbindgen:field-names=[mHandle, mNamespace]
/// * cbindgen:function-postfix=WR_DESTRUCTOR_SAFE
#[derive(Debug, Clone)]
pub struct DirectiveSet {
    directives: HashMap<String, DirectiveValue>
}

impl DirectiveSet {
    pub fn new() -> DirectiveSet {
        DirectiveSet {
            directives: HashMap::new(),
        }
    }

    pub fn parse(text: String) -> Result<DirectiveSet, String> {
        let mut directives = HashMap::new();

        for line in text.lines().map(|x| x.trim_left_matches("///").trim()) {
            if !line.starts_with("cbindgen:") {
                continue;
            }
            let directive = &line[9..];
            let parts: Vec<&str> = directive.split("=")
                                            .map(|x| x.trim())
                                            .collect();

            if parts.len() > 2 {
                return Err(format!("couldn't parse {}", line));
            }

            let name = parts[0];

            if parts.len() == 1 {
                directives.insert(name.to_string(), DirectiveValue::Bool(true));
                continue;
            }

            let value = parts[1];

            if let Some(x) = parse_list(value) {
                directives.insert(name.to_string(), DirectiveValue::List(x));
                continue;
            }
            if let Ok(x) = value.parse::<bool>() {
                directives.insert(name.to_string(), DirectiveValue::Bool(x));
                continue;
            }
            directives.insert(name.to_string(), if value.len() == 0 {
                DirectiveValue::Atom(None)
            } else {
                DirectiveValue::Atom(Some(value.to_string()))
            });
        }

        Ok(DirectiveSet {
            directives: directives
        })
    }

    pub fn list(&self, name: &str) -> Option<Vec<String>> {
        match self.directives.get(name) {
            Some(&DirectiveValue::List(ref x)) => Some(x.clone()),
            _ => None,
        }
    }
    pub fn atom(&self, name: &str) -> Option<Option<String>> {
        match self.directives.get(name) {
            Some(&DirectiveValue::Atom(ref x)) => Some(x.clone()),
            _ => None,
        }
    }
    pub fn bool(&self, name: &str) -> Option<bool> {
        match self.directives.get(name) {
            Some(&DirectiveValue::Bool(ref x)) => Some(*x),
            _ => None,
        }
    }
}

fn parse_list(list: &str) -> Option<Vec<String>> {
    if list.len() < 2 {
        return None;
    }

    match (list.chars().next(), list.chars().last()) {
        (Some('['), Some(']')) => {
            Some(list[1..list.len() - 1].split(',')
                                        .map(|x| x.trim().to_string())
                                        .collect())
        }
        _ => None
    }
}
