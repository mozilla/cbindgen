/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;

use crate::bindgen::ir::{GenericParams, ItemMap, Path, Struct, Type, Typedef};

/// Keeps track of named types that have the same underlying representation as some other type.
/// This happens via `#[repr(transparent)]` structs and via typedefs.
#[derive(Default)]
pub struct TransparentTypes {
    transparent: HashMap<Path, (Type, GenericParams)>,
}

impl TransparentTypes {
    pub fn add_structs(&mut self, structs: &ItemMap<Struct>) {
        structs.for_all_items(|s| {
            if s.is_transparent {
                self.transparent.insert(
                    s.path.clone(),
                    (s.fields[0].ty.clone(), s.generic_params.clone()),
                );
            }
        });
    }

    pub fn add_typedefs(&mut self, structs: &ItemMap<Typedef>) {
        structs.for_all_items(|t| {
            self.transparent.insert(
                t.path.clone(),
                (t.aliased.clone(), t.generic_params.clone()),
            );
        });
    }

    pub fn is_transparent(&self, ty: &Type) -> Option<Type> {
        let generic_path = match ty {
            Type::Path { path, .. } => path,
            _ => return None,
        };
        let (resolved, generic_params) = self.transparent.get(generic_path.path())?;
        let mappings = generic_params
            .iter()
            .zip(generic_path.generics())
            .collect::<Vec<_>>();
        Some(resolved.specialize(&mappings))
    }
}
