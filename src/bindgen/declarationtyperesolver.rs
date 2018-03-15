/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashSet;

pub struct DeclarationTypeResolver {
    structs: HashSet<String>,
    enums: HashSet<String>,
    unions: HashSet<String>,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum DeclarationType {
    Struct,
    Enum,
    Union,
}

impl DeclarationType {
    pub fn to_str(&self) -> &'static str {
        match self {
            &DeclarationType::Struct => "struct",
            &DeclarationType::Enum => "enum",
            &DeclarationType::Union => "union",
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

    pub fn add_enum(&mut self, name: &str) {
        self.enums.insert(name.to_owned());
    }

    pub fn add_struct(&mut self, name: &str) {
        self.structs.insert(name.to_owned());
    }

    pub fn add_union(&mut self, name: &str) {
        self.unions.insert(name.to_owned());
    }

    pub fn type_for(&self, name: &str) -> Option<DeclarationType> {
        if self.structs.contains(name) {
            Some(DeclarationType::Struct)
        } else if self.enums.contains(name) {
            Some(DeclarationType::Enum)
        } else if self.unions.contains(name) {
            Some(DeclarationType::Union)
        } else {
            None
        }
    }
}
