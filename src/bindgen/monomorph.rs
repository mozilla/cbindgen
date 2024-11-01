/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator as _;
use std::mem;

use crate::bindgen::ir::{
    Documentation, Enum, Field, GenericArgument, GenericPath, Item, ItemContainer, OpaqueItem,
    Path, Struct, Type, Typedef, Union,
};
use crate::bindgen::library::Library;

#[derive(Default, Clone, Debug)]
pub struct Monomorphs {
    replacements: HashMap<GenericPath, Path>,
    opaques: Vec<OpaqueItem>,
    structs: Vec<Struct>,
    unions: Vec<Union>,
    typedefs: Vec<Typedef>,
    enums: Vec<Enum>,
}

impl Monomorphs {
    pub fn contains(&self, path: &GenericPath) -> bool {
        self.replacements.contains_key(path)
    }

    pub fn insert_struct(
        &mut self,
        library: &Library,
        generic: &Struct,
        monomorph: Struct,
        arguments: Vec<GenericArgument>,
    ) {
        let replacement_path = GenericPath::new(generic.path.clone(), arguments);

        debug_assert!(generic.is_generic());
        debug_assert!(!self.contains(&replacement_path));

        self.replacements
            .insert(replacement_path, monomorph.path.clone());

        monomorph.add_monomorphs(library, self);

        self.structs.push(monomorph);
    }

    pub fn insert_enum(
        &mut self,
        library: &Library,
        generic: &Enum,
        monomorph: Enum,
        arguments: Vec<GenericArgument>,
    ) {
        let replacement_path = GenericPath::new(generic.path.clone(), arguments);

        debug_assert!(generic.is_generic());
        debug_assert!(!self.contains(&replacement_path));

        self.replacements
            .insert(replacement_path, monomorph.path.clone());

        monomorph.add_monomorphs(library, self);

        self.enums.push(monomorph);
    }

    pub fn insert_union(
        &mut self,
        library: &Library,
        generic: &Union,
        monomorph: Union,
        arguments: Vec<GenericArgument>,
    ) {
        let replacement_path = GenericPath::new(generic.path.clone(), arguments);

        debug_assert!(generic.is_generic());
        debug_assert!(!self.contains(&replacement_path));

        self.replacements
            .insert(replacement_path, monomorph.path.clone());

        monomorph.add_monomorphs(library, self);

        self.unions.push(monomorph);
    }

    pub fn insert_opaque(
        &mut self,
        generic: &OpaqueItem,
        monomorph: OpaqueItem,
        arguments: Vec<GenericArgument>,
    ) {
        let replacement_path = GenericPath::new(generic.path.clone(), arguments);

        debug_assert!(generic.is_generic());
        debug_assert!(!self.contains(&replacement_path));

        self.replacements
            .insert(replacement_path, monomorph.path.clone());
        self.opaques.push(monomorph);
    }

    pub fn insert_typedef(
        &mut self,
        library: &Library,
        generic: &Typedef,
        monomorph: Typedef,
        arguments: Vec<GenericArgument>,
    ) {
        let replacement_path = GenericPath::new(generic.path.clone(), arguments);

        debug_assert!(generic.is_generic());
        debug_assert!(!self.contains(&replacement_path));

        self.replacements
            .insert(replacement_path, monomorph.path.clone());

        monomorph.add_monomorphs(library, self);

        self.typedefs.push(monomorph);
    }

    pub fn mangle_path(&self, path: &GenericPath) -> Option<&Path> {
        self.replacements.get(path)
    }

    pub fn drain_opaques(&mut self) -> Vec<OpaqueItem> {
        mem::take(&mut self.opaques)
    }

    pub fn drain_structs(&mut self) -> Vec<Struct> {
        mem::take(&mut self.structs)
    }

    pub fn drain_unions(&mut self) -> Vec<Union> {
        mem::take(&mut self.unions)
    }

    pub fn drain_typedefs(&mut self) -> Vec<Typedef> {
        mem::take(&mut self.typedefs)
    }

    pub fn drain_enums(&mut self) -> Vec<Enum> {
        mem::take(&mut self.enums)
    }
}

/// A helper for collecting all function return value momomorphs -- template types returned by
/// functions that can lead to compilation warnings/errors if not explicitly instantiated.
pub struct ReturnValueMonomorphs<'a> {
    library: &'a Library,
    monomorphs: HashSet<GenericPath>,
}

impl<'a> ReturnValueMonomorphs<'a> {
    pub fn new(library: &'a Library) -> Self {
        Self {
            library,
            monomorphs: HashSet::new(),
        }
    }

    /// Resolve a typedef that is a function return value, specializing it first if needed.
    fn handle_return_value_typedef(&mut self, typedef: Typedef, generic: &GenericPath) {
        if typedef.is_generic() {
            let args = generic.generics();
            let aliased = &typedef.aliased;
            let mappings = typedef.generic_params.call(typedef.path.name(), args);
            let aliased = aliased.specialize(&mappings);
            aliased.find_return_value_monomorphs(self, true);
        } else {
            typedef.find_return_value_monomorphs(self, true);
        }
    }

    /// Once we find a function return type, what we do with it depends on the type of item it
    /// resolves to. Typedefs need to be resolved recursively, while generic structs, unions, and
    /// enums are captured in the set of return value monomorphs.
    pub fn handle_return_value_path(&mut self, generic: &GenericPath, is_return_value: bool) {
        if !is_return_value {
            return;
        }

        for item in self.library.get_items(generic.path()).into_iter().flatten() {
            match item {
                // Constants and statics cannot be function return types
                ItemContainer::Constant(_) | ItemContainer::Static(_) => {}
                // Opaque items cannot be instantiated (doomed to compilation failure)
                ItemContainer::OpaqueItem(_) => {}
                ItemContainer::Typedef(t) => self.handle_return_value_typedef(t, generic),
                ItemContainer::Union(_) | ItemContainer::Enum(_) => {
                    if !generic.generics().is_empty() {
                        self.monomorphs.insert(generic.clone());
                    }
                }
                ItemContainer::Struct(s) => {
                    if let Some(t) = s.as_typedef() {
                        self.handle_return_value_typedef(t, generic);
                    } else if !generic.generics().is_empty() {
                        self.monomorphs.insert(generic.clone());
                    }
                }
            }
        }
    }

    /// Whenever we encounter a function (or function pointer), we need to check whether its return
    /// value is an instantiated generic type (monomorph).
    pub fn handle_function<'i>(&mut self, ret: &Type, args: impl IntoIterator<Item = &'i Type>) {
        ret.find_return_value_monomorphs(self, true);
        for arg in args.into_iter() {
            arg.find_return_value_monomorphs(self, false);
        }
    }

    /// Emit all instantiated return value monomorphs as fields of a dummy struct, which silences
    /// warnings and errors on several compilers.
    pub fn into_struct(self, struct_name: &str) -> Option<(Type, Struct)> {
        if self.monomorphs.is_empty() {
            return None;
        }

        // Sort the output so that the struct remains stable across runs (tests want that).
        let mut monomorphs = Vec::from_iter(self.monomorphs);
        monomorphs.sort();
        let fields = monomorphs
            .into_iter()
            .enumerate()
            .map(|(i, path)| Field::from_name_and_type(format!("field{}", i), Type::Path(path)))
            .collect();
        let doc_comment = vec![
            " Dummy struct emitted by cbindgen to avoid compiler warnings/errors about",
            " return type C linkage for template types returned by value from functions",
        ];
        let doc_comment = doc_comment.into_iter().map(Into::into).collect();
        let struct_name = GenericPath::new(Path::new(struct_name), vec![]);
        let struct_def = Struct::new(
            struct_name.path().clone(),
            Default::default(), // no generic params
            fields,
            false,              // no tag field
            false,              // not an enum body
            None,               // no special alignment requirements
            false,              // not transparent
            None,               // no conf
            Default::default(), // no annotations
            Documentation { doc_comment },
        );
        let struct_name = Type::Path(struct_name);
        Some((struct_name, struct_def))
    }
}
