/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;
use std::str::FromStr;

use syn;

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

    pub fn is_empty(&self) -> bool {
        self.annotations.is_empty()
    }

    pub fn load(attrs: &Vec<syn::Attribute>) -> Result<AnnotationSet, String> {
        let mut lines = Vec::new();

        for attr in attrs {
            if attr.style == syn::AttrStyle::Outer &&
               attr.is_sugared_doc {
                if let syn::MetaItem::NameValue(_, syn::Lit::Str(ref comment, _)) = attr.value {
                    let line = comment.trim_left_matches("///").trim();

                    if line.starts_with("cbindgen:") {
                        lines.push(line.to_owned());
                    }
                }
            }
        }

        let mut annotations = HashMap::new();

        // Look at each line for an annotation
        for line in lines {
            // Skip lines that don't start with cbindgen
            if !line.starts_with("cbindgen:") {
                continue;
            }

            // Remove the "cbingen:" prefix
            let annotation = &line[9..];

            // Split the annotation in two
            let parts: Vec<&str> = annotation.split("=")
                                            .map(|x| x.trim())
                                            .collect();

            if parts.len() > 2 {
                return Err(format!("couldn't parse {}", line));
            }

            // Grab the name that this annotation is modifying
            let name = parts[0];

            // If the annotation only has a name, assume it's setting a bool flag
            if parts.len() == 1 {
                annotations.insert(name.to_string(), AnnotationValue::Bool(true));
                continue;
            }

            // Parse the value we're setting the name to
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

/// Parse lists like "[x, y, z]". This is not implemented efficiently or well.
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
