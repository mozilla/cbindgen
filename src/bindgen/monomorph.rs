/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;
use std::collections::BTreeMap;
use std::mem;

use bindgen::dependencies::Dependencies;
use bindgen::ir::{GenericPath, OpaqueItem, Path, Struct, Type, Union};
use bindgen::library::Library;

#[derive(Clone, Debug)]
pub struct TemplateSpecialization {
    pub generic: Struct,
    pub monomorphs: Vec<(Path, Vec<Type>)>,
}

impl TemplateSpecialization {
    fn new(generic: Struct) -> TemplateSpecialization {
        TemplateSpecialization {
            generic: generic,
            monomorphs: Vec::new(),
        }
    }

    pub fn add_dependencies(&self, library: &Library, out: &mut Dependencies) {
            for &(_, ref generic_values) in &self.monomorphs {
                for generic_value in generic_values {
                    generic_value.add_dependencies(library, out);
                }
            }
    }
}

#[derive(Clone, Debug)]
pub struct Monomorphs {
    replacements: HashMap<GenericPath, Path>,
    opaques: Vec<OpaqueItem>,
    structs: Vec<Struct>,
    unions: Vec<Union>,
    templates: BTreeMap<Path, TemplateSpecialization>,
}

impl Monomorphs {
    pub fn new() -> Monomorphs {
        Monomorphs {
            replacements: HashMap::new(),
            opaques: Vec::new(),
            structs: Vec::new(),
            unions: Vec::new(),
            templates: BTreeMap::new(),
        }
    }

    pub fn contains(&self, path: &GenericPath) -> bool {
        self.replacements.contains_key(path)
    }

    pub fn insert_struct(&mut self,
                         generic: &Struct,
                         monomorph: Struct,
                         parameters: Vec<Type>) {
        // Add extra information for struct instantiations so we can use template
        // specialization to make using the type more ergonomic.
        self.templates.entry(generic.name.clone())
                             .or_insert_with(|| TemplateSpecialization::new(generic.clone()))
                             .monomorphs.push((monomorph.name.clone(), parameters.clone()));

        let replacement_path = GenericPath::new(generic.name.clone(), parameters);

        debug_assert!(generic.generic_params.len() > 0);
        debug_assert!(!self.contains(&replacement_path));

        self.replacements.insert(replacement_path, monomorph.name.clone());
        self.structs.push(monomorph);
    }

    pub fn insert_union(&mut self,
                        generic: &Union,
                        monomorph: Union,
                        parameters: Vec<Type>) {
        let replacement_path = GenericPath::new(generic.name.clone(), parameters);

        debug_assert!(generic.generic_params.len() > 0);
        debug_assert!(!self.contains(&replacement_path));

        self.replacements.insert(replacement_path, monomorph.name.clone());
        self.unions.push(monomorph);
    }

    pub fn insert_opaque(&mut self,
                         generic: &OpaqueItem,
                         monomorph: OpaqueItem,
                         parameters: Vec<Type>) {
        let replacement_path = GenericPath::new(generic.name.clone(), parameters);

        debug_assert!(generic.generic_params.len() > 0);
        debug_assert!(!self.contains(&replacement_path));

        self.replacements.insert(replacement_path, monomorph.name.clone());
        self.opaques.push(monomorph);
    }

    pub fn mangle_path(&self, path: &GenericPath) -> Option<&Path> {
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

    pub fn drain_template_specializations(&mut self) -> Vec<TemplateSpecialization> {
        let mut not_mangled = mem::replace(&mut self.templates, BTreeMap::new());
        let mut mangled = Vec::new();

        // The generic type arguments in `templates` need to be mangled
        for (_, template) in &mut not_mangled {
            for &mut (_, ref mut generic_values) in &mut template.monomorphs {
                for generic_value in generic_values {
                    generic_value.mangle_paths(&self);
                }
            }

            mangled.push(template.clone());
        }

        mangled
    }
}
