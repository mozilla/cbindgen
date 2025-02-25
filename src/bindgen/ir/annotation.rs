/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::borrow::Cow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::str::FromStr;

use crate::bindgen::config::{Config, Language};
use crate::bindgen::utilities::SynAttributeHelpers;
use regex::Regex;

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
    Dict(HashMap<String, AnnotationValue>),
}

/// A set of annotations specified by a document comment.
#[derive(Debug, Default, Clone)]
pub struct AnnotationSet {
    annotations: HashMap<String, AnnotationValue>,
    pub must_use: bool,
    pub deprecated: Option<String>,
}

pub enum DeprecatedNoteKind {
    Function,
    Struct,
    Enum,
    EnumVariant,
}

impl AnnotationSet {
    pub fn new() -> AnnotationSet {
        AnnotationSet {
            annotations: HashMap::new(),
            must_use: false,
            deprecated: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.annotations.is_empty() && !self.must_use
    }

    pub(crate) fn must_use(&self, config: &Config) -> bool {
        self.must_use && config.language != Language::Cython
    }

    pub(crate) fn deprecated_note<'c>(
        &self,
        config: &'c Config,
        kind: DeprecatedNoteKind,
    ) -> Option<Cow<'c, str>> {
        let note = self.deprecated.as_deref()?;

        if config.language == Language::Cython {
            return None;
        }

        if note.is_empty() {
            return Some(Cow::Borrowed(match kind {
                DeprecatedNoteKind::Enum => config.enumeration.deprecated.as_deref()?,
                DeprecatedNoteKind::EnumVariant => {
                    config.enumeration.deprecated_variant.as_deref()?
                }
                DeprecatedNoteKind::Function => config.function.deprecated.as_deref()?,
                DeprecatedNoteKind::Struct => config.structure.deprecated.as_deref()?,
            }));
        }

        let format = match kind {
            DeprecatedNoteKind::Enum => &config.enumeration.deprecated_with_note,
            DeprecatedNoteKind::EnumVariant => &config.enumeration.deprecated_variant_with_note,
            DeprecatedNoteKind::Function => &config.function.deprecated_with_note,
            DeprecatedNoteKind::Struct => &config.structure.deprecated_with_note,
        }
        .as_ref()?;
        Some(Cow::Owned(format.replace("{}", &format!("{:?}", note))))
    }

    pub fn load(attrs: &[syn::Attribute]) -> Result<AnnotationSet, String> {
        let lines = attrs.get_comment_lines();
        let lines: Vec<&str> = lines
            .iter()
            .filter_map(|line| {
                let line = line.trim_start();
                if !line.starts_with("cbindgen:") {
                    return None;
                }

                Some(line)
            })
            .collect();

        let must_use = attrs.has_attr_word("must_use");
        let deprecated = attrs.find_deprecated_note();
        let mut annotations = HashMap::new();

        // Regex to extract the index name from an annotation
        let annotation_name_regex = Regex::new(r"(?m)([a-zA-Z0-9_-]+)(\[([a-zA-Z0-9_-]+)\])?")
            .expect("Failed to build annotation regex!");

        // Look at each line for an annotation
        for line in lines {
            debug_assert!(line.starts_with("cbindgen:"));

            // Remove the "cbindgen:" prefix
            let annotation = &line[9..];

            // Split the annotation in two
            let parts: Vec<&str> = annotation.split('=').map(|x| x.trim()).collect();

            if parts.len() > 2 {
                return Err(format!("Couldn't parse {}.", line));
            }

            let captures = annotation_name_regex.captures(parts[0]).ok_or_else(|| {
                format!("Couldn't parse annotation {:?} in line {}", parts[0], line)
            })?;

            // Grab the name that this annotation is modifying
            let name = captures
                .get(1)
                .ok_or_else(|| {
                    format!("Couldn't parse annotation {:?} in line {}", parts[0], line)
                })?
                .as_str();

            // Check if this annotation is a dictionary
            let index = captures.get(3).map(|capture| capture.as_str());

            // Parse the value we're setting the name to
            let value = if parts.len() == 1 {
                // If the annotation only has a name, assume it's setting a bool flag
                AnnotationValue::Bool(true)
            } else {
                parse_value(parts[1])
            };

            match index {
                Some(index) => {
                    // Create a new dictionary entry if it doesn't exist
                    let entry = annotations
                        .entry(name.to_string())
                        .or_insert(AnnotationValue::Dict(HashMap::new()));

                    match entry {
                        AnnotationValue::Dict(ref mut dict) => {
                            dict.insert(index.to_string(), value);
                        }
                        _ => {
                            // This is here so a mistyped cbindgen:foo[bar]=baz doesn't silently discard all previous dictionary entries.
                            return Err(format!(
                                "Attempted to change type of annotation {} in line {}",
                                name, line
                            ));
                        }
                    }
                }
                None => {
                    annotations.insert(name.to_string(), value);
                }
            }
        }

        Ok(AnnotationSet {
            annotations,
            must_use,
            deprecated,
        })
    }

    /// Adds an annotation value if none is specified.
    pub fn add_default(&mut self, name: &str, value: AnnotationValue) {
        if let Entry::Vacant(e) = self.annotations.entry(name.to_string()) {
            e.insert(value);
        }
    }

    pub fn list(&self, name: &str) -> Option<Vec<String>> {
        match self.annotations.get(name) {
            Some(AnnotationValue::List(x)) => Some(x.clone()),
            _ => None,
        }
    }
    pub fn atom(&self, name: &str) -> Option<Option<String>> {
        match self.annotations.get(name) {
            Some(AnnotationValue::Atom(x)) => Some(x.clone()),
            _ => None,
        }
    }
    pub fn bool(&self, name: &str) -> Option<bool> {
        match self.annotations.get(name) {
            Some(AnnotationValue::Bool(x)) => Some(*x),
            _ => None,
        }
    }

    pub fn dict(&self, name: &str) -> Option<HashMap<String, AnnotationValue>> {
        match self.annotations.get(name) {
            Some(AnnotationValue::Dict(x)) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn parse_atom<T>(&self, name: &str) -> Option<T>
    where
        T: Default + FromStr,
    {
        match self.annotations.get(name) {
            Some(AnnotationValue::Atom(x)) => Some(
                x.as_ref()
                    .map_or(T::default(), |y| y.parse::<T>().ok().unwrap()),
            ),
            _ => None,
        }
    }
}

/// Parse a value into an annotation value.
fn parse_value(value: &str) -> AnnotationValue {
    if let Some(x) = parse_list(value) {
        return AnnotationValue::List(x);
    }
    if let Ok(x) = value.parse::<bool>() {
        return AnnotationValue::Bool(x);
    }
    if value.is_empty() {
        return AnnotationValue::Atom(None);
    }
    AnnotationValue::Atom(Some(value.to_string()))
}

/// Parse lists like "[x, y, z]". This is not implemented efficiently or well.
fn parse_list(list: &str) -> Option<Vec<String>> {
    // Remove leading and trailing whitespace
    let list = list.trim();

    // Ensure that the list is at least 2 characters long
    if list.len() < 2 {
        return None;
    }

    // Ensure that the list starts and ends with brackets
    match (list.chars().next(), list.chars().last()) {
        (Some('['), Some(']')) => {}
        _ => return None,
    }

    let mut items = Vec::new();
    let mut current = String::new();
    let mut escape = false;

    for c in list[1..list.len() - 1].chars() {
        if escape {
            current.push(c);
            escape = false;
        } else if c == '\\' {
            escape = true;
        } else if c == ',' {
            items.push(current.trim().to_string());
            current.clear();
        } else {
            current.push(c);
        }
    }

    if !current.is_empty() {
        items.push(current.trim().to_string());
    }

    Some(items)
}
