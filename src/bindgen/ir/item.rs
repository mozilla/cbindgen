/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use bindgen::dependencies::Dependencies;
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
    pub fn add_dependencies(&self, library: &Library, out: &mut Dependencies) {
        match self {
            &Item::Struct(ref x) => {
                x.add_dependencies(library, out);
            },
            &Item::Typedef(ref x) => {
                x.add_dependencies(library, out);
            },
            &Item::Specialization(..) => {
                unreachable!();
            },
            _ => { }
        }
    }
}
