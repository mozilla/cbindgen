/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashSet;
use std::cmp::Ordering;

use bindgen::ir::{Item, Path};

/// A dependency list is used for gathering what order to output the types.
pub struct Dependencies {
    pub order: Vec<Item>,
    pub items: HashSet<Path>,
}

impl Dependencies {
    pub fn new() -> Dependencies {
        Dependencies {
            order: Vec::new(),
            items: HashSet::new(),
        }
    }

    pub fn sort(&mut self) {
        // Sort enums and opaque structs into their own layers because they don't
        // depend on each other or anything else.
        let ordering = |a: &Item, b: &Item| {
            match (a, b) {
                (&Item::Enum(ref x), &Item::Enum(ref y)) => x.name.cmp(&y.name),
                (&Item::Enum(_), _) => Ordering::Less,
                (_, &Item::Enum(_)) => Ordering::Greater,

                (&Item::OpaqueItem(ref x), &Item::OpaqueItem(ref y)) => x.name.cmp(&y.name),
                (&Item::OpaqueItem(_), _) => Ordering::Less,
                (_, &Item::OpaqueItem(_)) => Ordering::Greater,

                _ => Ordering::Equal,
            }
        };

        self.order.sort_by(ordering);
    }
}
