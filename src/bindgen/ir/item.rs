/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use bindgen::config::*;
use bindgen::ir::*;
use bindgen::library::*;

/// An item is any type of rust item besides a function
#[derive(Debug, Clone)]
pub enum Item {
    OpaqueItem(OpaqueItem),
    Struct(Struct),
    Enum(Enum),
    Typedef(Typedef),
    Specialization(Specialization),
}

impl Item {
    pub fn name(&self) -> &str {
        match self {
            &Item::OpaqueItem(ref x) => { &x.name },
            &Item::Struct(ref x) => { &x.name },
            &Item::Enum(ref x) => { &x.name },
            &Item::Typedef(ref x) => { &x.name },
            &Item::Specialization(ref x) => { &x.name },
        }
    }

    pub fn add_deps(&self, library: &Library, out: &mut DependencyList) {
        match self {
            &Item::Struct(ref x) => {
                x.add_deps(library, out);
            },
            &Item::Typedef(ref x) => {
                x.add_deps(library, out);
            },
            &Item::Specialization(..) => {
                unreachable!();
            },
            _ => { }
        }
    }

    pub fn add_specializations(&self, library: &Library, out: &mut SpecializationList) {
        match self {
            &Item::Struct(ref x) => {
                x.add_specializations(library, out);
            },
            &Item::Typedef(ref x) => {
                x.add_specializations(library, out);
            },
            &Item::Specialization(ref x) => {
                x.add_specializations(library, out);
            },
            _ => { }
        }
    }

    pub fn rename_fields(&mut self, config: &Config) {
        match self {
            &mut Item::Struct(ref mut x) => { x.rename_fields(config); },
            &mut Item::Enum(ref mut x) => { x.rename_fields(config); },
            _ => { },
        }
    }

    pub fn mangle_paths(&mut self, monomorphs: &Monomorphs) {
        match self {
            &mut Item::OpaqueItem(_) => { },
            &mut Item::Struct(ref mut x) => {
                x.mangle_paths(monomorphs);
            },
            &mut Item::Enum(_) => { },
            &mut Item::Typedef(ref mut x) => {
                x.mangle_paths(monomorphs);
            },
            &mut Item::Specialization(..) => {
                unreachable!();
            },
        }
    }
}
