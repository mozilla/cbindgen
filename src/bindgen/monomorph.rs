/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;
use std::mem;

use bindgen::ir::{Enum, Generic, OpaqueItem, Path, Struct, Type, Typedef, Union};

#[derive(Default, Clone, Debug)]
pub struct Monomorphs {
    replacements: HashMap<Generic, Path>,
    opaques: Vec<OpaqueItem>,
    structs: Vec<Struct>,
    unions: Vec<Union>,
    typedefs: Vec<Typedef>,
    enums: Vec<Enum>,
}

impl Monomorphs {
    pub fn contains(&self, path: &Generic) -> bool {
        self.replacements.contains_key(path)
    }

    pub fn insert_struct(&mut self, generic: &Struct, monomorph: Struct, parameters: Vec<Type>) {
        let replacement_path = Generic::new(generic.path.clone(), parameters);

        debug_assert!(generic.generic_params.len() > 0);
        debug_assert!(!self.contains(&replacement_path));

        self.replacements
            .insert(replacement_path, monomorph.path.clone());
        self.structs.push(monomorph);
    }

    pub fn insert_enum(&mut self, generic: &Enum, monomorph: Enum, parameters: Vec<Type>) {
        let replacement_path = Generic::new(generic.path.clone(), parameters);

        debug_assert!(generic.generic_params.len() > 0);
        debug_assert!(!self.contains(&replacement_path));

        self.replacements
            .insert(replacement_path, monomorph.path.clone());
        self.enums.push(monomorph);
    }

    pub fn insert_union(&mut self, generic: &Union, monomorph: Union, parameters: Vec<Type>) {
        let replacement_path = Generic::new(generic.path.clone(), parameters);

        debug_assert!(generic.generic_params.len() > 0);
        debug_assert!(!self.contains(&replacement_path));

        self.replacements
            .insert(replacement_path, monomorph.path.clone());
        self.unions.push(monomorph);
    }

    pub fn insert_opaque(
        &mut self,
        generic: &OpaqueItem,
        monomorph: OpaqueItem,
        parameters: Vec<Type>,
    ) {
        let replacement_path = Generic::new(generic.path.clone(), parameters);

        debug_assert!(generic.generic_params.len() > 0);
        debug_assert!(!self.contains(&replacement_path));

        self.replacements
            .insert(replacement_path, monomorph.path.clone());
        self.opaques.push(monomorph);
    }

    pub fn insert_typedef(&mut self, generic: &Typedef, monomorph: Typedef, parameters: Vec<Type>) {
        let replacement_path = Generic::new(generic.path.clone(), parameters);

        debug_assert!(generic.generic_params.len() > 0);
        debug_assert!(!self.contains(&replacement_path));

        self.replacements
            .insert(replacement_path, monomorph.path.clone());
        self.typedefs.push(monomorph);
    }

    pub fn mangle_path(&self, path: &Generic) -> Option<&Path> {
        self.replacements.get(path)
    }

    pub fn drain_opaques(&mut self) -> Vec<OpaqueItem> {
        mem::replace(&mut self.opaques, Vec::new())
    }

    pub fn drain_structs(&mut self) -> Vec<Struct> {
        mem::replace(&mut self.structs, Vec::new())
    }

    pub fn drain_unions(&mut self) -> Vec<Union> {
        mem::replace(&mut self.unions, Vec::new())
    }

    pub fn drain_typedefs(&mut self) -> Vec<Typedef> {
        mem::replace(&mut self.typedefs, Vec::new())
    }

    pub fn drain_enums(&mut self) -> Vec<Enum> {
        mem::replace(&mut self.enums, Vec::new())
    }
}
