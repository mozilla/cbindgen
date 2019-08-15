/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashSet;

use bindgen::ir::Path;

pub struct DeclarationTypeResolver {
    structs: HashSet<Path>,
    enums: HashSet<Path>,
    unions: HashSet<Path>,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum DeclarationType {
    Struct,
    Enum,
    Union,
}

impl DeclarationType {
    pub fn to_str(self) -> &'static str {
        match self {
            DeclarationType::Struct => "struct",
            DeclarationType::Enum => "enum",
            DeclarationType::Union => "union",
        }
    }
}

impl DeclarationTypeResolver {
    pub fn new() -> DeclarationTypeResolver {
        DeclarationTypeResolver {
            structs: HashSet::new(),
            enums: HashSet::new(),
            unions: HashSet::new(),
        }
    }

    pub fn add_enum(&mut self, path: &Path) {
        self.enums.insert(path.clone());
    }

    pub fn add_struct(&mut self, path: &Path) {
        self.structs.insert(path.clone());
    }

    pub fn add_union(&mut self, path: &Path) {
        self.unions.insert(path.clone());
    }

    pub fn type_for(&self, path: &Path) -> Option<DeclarationType> {
        // FIXME: don't look up by name, but by full path:
        if self.structs.contains(path) {
            Some(DeclarationType::Struct)
        } else if self.enums.contains(path) {
            Some(DeclarationType::Enum)
        } else if self.unions.contains(path) {
            Some(DeclarationType::Union)
        } else {
            None
        }
    }
}
