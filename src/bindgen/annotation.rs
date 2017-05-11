use std::collections::HashMap;
use std::str::FromStr;

// A system for specifying properties on items. Annotations are
// given through document comments and parsed by this code.
//
// An annotation is in the form cbindgen:PROPERTY=VALUE
// Where PROPERTY depends on the item
// Where VALUE can be
//  * list - [Item1, Item2, Item3, ...]
//  * atom - Foo
//  * bool - true,false
// Examples:
//  * cbindgen:field-names=[mHandle, mNamespace]
//  * cbindgen:function-postfix=WR_DESTRUCTOR_SAFE

/// A value specified by an annotation.
#[derive(Debug, Clone)]
pub enum AnnotationValue {
    List(Vec<String>),
    Atom(Option<String>),
    Bool(bool),
}

/// A set of annotations specified by a document comment.
#[derive(Debug, Clone)]
pub struct AnnotationSet {
    annotations: HashMap<String, AnnotationValue>
}

impl AnnotationSet {
    pub fn new() -> AnnotationSet {
        AnnotationSet {
            annotations: HashMap::new(),
        }
    }

    pub fn parse(text: String) -> Result<AnnotationSet, String> {
        let mut annotations = HashMap::new();

        for line in text.lines().map(|x| x.trim_left_matches("///").trim()) {
            if !line.starts_with("cbindgen:") {
                continue;
            }
            let annotation = &line[9..];
            let parts: Vec<&str> = annotation.split("=")
                                            .map(|x| x.trim())
                                            .collect();

            if parts.len() > 2 {
                return Err(format!("couldn't parse {}", line));
            }

            let name = parts[0];

            if parts.len() == 1 {
                annotations.insert(name.to_string(), AnnotationValue::Bool(true));
                continue;
            }

            let value = parts[1];

            if let Some(x) = parse_list(value) {
                annotations.insert(name.to_string(), AnnotationValue::List(x));
                continue;
            }
            if let Ok(x) = value.parse::<bool>() {
                annotations.insert(name.to_string(), AnnotationValue::Bool(x));
                continue;
            }
            annotations.insert(name.to_string(), if value.len() == 0 {
                AnnotationValue::Atom(None)
            } else {
                AnnotationValue::Atom(Some(value.to_string()))
            });
        }

        Ok(AnnotationSet {
            annotations: annotations
        })
    }

    pub fn list(&self, name: &str) -> Option<Vec<String>> {
        match self.annotations.get(name) {
            Some(&AnnotationValue::List(ref x)) => Some(x.clone()),
            _ => None,
        }
    }
    pub fn atom(&self, name: &str) -> Option<Option<String>> {
        match self.annotations.get(name) {
            Some(&AnnotationValue::Atom(ref x)) => Some(x.clone()),
            _ => None,
        }
    }
    pub fn bool(&self, name: &str) -> Option<bool> {
        match self.annotations.get(name) {
            Some(&AnnotationValue::Bool(ref x)) => Some(*x),
            _ => None,
        }
    }

    pub fn parse_atom<T>(&self, name: &str) -> Option<T>
        where T: Default + FromStr
    {
        match self.annotations.get(name) {
            Some(&AnnotationValue::Atom(ref x)) => {
                Some(x.as_ref().map_or(T::default(), |y| { y.parse::<T>().ok().unwrap() }))
            }
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
