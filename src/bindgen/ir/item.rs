/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use bindgen::ir::{Enum, OpaqueItem, Specialization, Struct, Typedef, Function};
use bindgen::dependencies::DependencyKind;
use bindgen::library::Library;
use bindgen::writer::{Source, SourceWriter};
use bindgen::config::Config;
use std::hash::{Hash, Hasher};
use std::fmt::{self, Display};
use std::io::Write;

/// An item is any type of rust item
#[derive(Debug, Clone)]
pub enum Item {
    Enum(Enum),
    Struct(Struct),
    OpaqueItem(OpaqueItem),
    Typedef(Typedef),
    Function(Function),
    Specialization(Specialization),
}

impl Item {
    fn name(&self) -> &str {
        match *self {
            Item::Enum(ref e) => &e.name,
            Item::Struct(ref s) => &s.name,
            Item::OpaqueItem(ref o) => &o.name,
            Item::Typedef(ref t) => &t.name,
            Item::Function(ref f) => &f.name,
            Item::Specialization(ref s) => &s.name,
        }
    }

    pub fn get_deps(&self, library: &Library) -> Vec<(Item, DependencyKind)> {
        match *self {
            Item::Enum(_) | Item::OpaqueItem(_) => Vec::new(),
            Item::Specialization(ref s) => s.get_deps(library),
            Item::Struct(ref s) => s.get_deps(library),
            Item::Typedef(ref t) => t.get_deps(library),
            Item::Function(ref f) => f.get_deps(library),
        }
    }

    pub fn mangle_paths(&mut self) {
        match *self {
            Item::Enum(_) | Item::OpaqueItem(_) | Item::Specialization(_) => {}
            Item::Struct(ref mut s) => s.mangle_paths(),
            Item::Typedef(ref mut t) => t.mangle_paths(),
            Item::Function(ref mut f) => f.mangle_paths(),

        }
    }

    pub fn apply_transformation(&mut self, config: &Config) {
        match *self {
            Item::Enum(ref mut e) => {
                e.rename_values(config);
            }
            Item::Struct(ref mut s) => {
                s.rename_fields(config);
            }
            Item::Function(ref mut f) => {
                f.rename_args(config);
            }
            _ => {}
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Item::Enum(ref e) => write!(f, "Enum {}", e.name),
            Item::Struct(ref s) => write!(f, "Struct {}", s.name),
            Item::OpaqueItem(ref o) => write!(f, "Opaque {}", o.name),
            Item::Typedef(ref t) => write!(f, "Typedef {}", t.name),
            Item::Function(ref c) => write!(f, "Function {}", c.name),
            Item::Specialization(ref s) if !s.generic_values.is_empty() => {
                write!(f, "Specialization {}<", s.name)?;
                let mut first = true;
                for g in &s.generic_values {
                    if first {
                        first = false;
                    } else {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}", g.get_root_path())?;
                }
                write!(f, ">")
            }
            Item::Specialization(ref s) => {
                write!(f, "Specialization {}<", s.name)?;
                let mut first = true;
                for g in &s.generic_params {
                    if first {
                        first = false;
                    } else {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", g)?;
                }
                write!(f, ">")
            }
        }
    }
}

impl Hash for Item {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.name().hash(state);
        match *self {
            Item::Enum(_) => "enum".hash(state),
            Item::Struct(_) => "struct".hash(state),
            Item::OpaqueItem(_) => "opaque".hash(state),
            Item::Typedef(_) => "typedef".hash(state),
            Item::Function(_) => "function".hash(state),
            Item::Specialization(ref s) => {
                "specialization".hash(state);
                s.generic_values.hash(state);
            }
        }
    }
}

impl PartialEq<Self> for Item {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (&Item::Enum(ref e1), &Item::Enum(ref e2)) => e1.name == e2.name,
            (&Item::Struct(ref s1), &Item::Struct(ref s2)) => s1.name == s2.name,
            (&Item::OpaqueItem(ref o1), &Item::OpaqueItem(ref o2)) => o1.name == o2.name,
            (&Item::Typedef(ref t1), &Item::Typedef(ref t2)) => t1.name == t2.name,
            (&Item::Function(ref f1), &Item::Function(ref f2)) => f1.name == f2.name,
            (&Item::Specialization(ref s1), &Item::Specialization(ref s2)) => {
                s1.name == s2.name && s1.generic_values == s2.generic_values
            }
            _ => false,
        }
    }
}

impl Eq for Item {}

impl Source for Item {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        match *self {
            Item::Enum(ref e) => e.write(config, out),
            Item::Struct(ref s) => s.write(config, out),
            Item::OpaqueItem(ref o) => o.write(config, out),
            Item::Typedef(ref t) => t.write(config, out),
            Item::Function(ref f) => f.write(config, out),
            Item::Specialization(ref s) => s.write(config, out),
        }
    }
}
