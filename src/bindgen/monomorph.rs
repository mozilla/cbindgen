/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;
use std::mem;

use crate::bindgen::ir::{
    Enum, Field, GenericArgument, GenericPath, Item, OpaqueItem, Path, Struct, Typedef, Union,
    VariantBody,
};
use crate::bindgen::ir::{GenericParams, Type};
use crate::bindgen::library::Library;

#[derive(Default, Clone, Debug)]
pub struct Monomorphs {
    replacements: HashMap<GenericPath, Path>,
    opaques: Vec<OpaqueItem>,
    // Pairs of (generic item, its monomorph) needed for subsequent C++ validity check
    structs: Vec<(Struct, Struct)>,
    unions: Vec<(Union, Union)>,
    enums: Vec<(Enum, Enum)>,
    typedefs: Vec<(Typedef, Typedef)>,
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

        self.structs.push((generic.clone(), monomorph));
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

        self.enums.push((generic.clone(), monomorph));
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

        self.unions.push((generic.clone(), monomorph));
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
        self.register_typedef(generic, &monomorph.path, arguments);
        monomorph.add_monomorphs(library, self);
        self.typedefs.push((generic.clone(), monomorph));
    }

    // Only registers a typedef instantiation for name mangling purposes
    pub fn register_typedef(
        &mut self,
        generic: &Typedef,
        monomorph_path: &Path,
        arguments: Vec<GenericArgument>,
    ) {
        let replacement_path = GenericPath::new(generic.path.clone(), arguments);

        debug_assert!(generic.is_generic());
        debug_assert!(!self.contains(&replacement_path));

        self.replacements
            .insert(replacement_path, monomorph_path.clone());
    }

    /// C++ bindings keep generic templates and write zero-sized generic arguments as `void`.
    /// Instantiations that lose fields or function arguments due to this
    /// can't be represented correctly, so warn about them.
    pub fn warn_zst_instantiations(&self) {
        for (g, m) in &self.structs {
            // Variant bodies are reported by the enum loop below, under the enum's name.
            if g.is_enum_variant_body {
                continue;
            }
            Self::warn_missing(
                g.path.name(),
                m.path.name(),
                &g.fields,
                &m.fields,
                g.generic_params(),
            );
        }
        for (g, m) in &self.unions {
            Self::warn_missing(
                g.path.name(),
                m.path.name(),
                &g.fields,
                &m.fields,
                g.generic_params(),
            );
        }
        for (g, m) in &self.enums {
            for (gv, mv) in g.variants.iter().zip(&m.variants) {
                if let (VariantBody::Body { body: gb, .. }, VariantBody::Body { body: mb, .. }) =
                    (&gv.body, &mv.body)
                {
                    Self::warn_missing(
                        &format!("{}::{}", g.path, gv.name),
                        &format!("{}::{}", m.path, mv.name),
                        &gb.fields,
                        &mb.fields,
                        gb.generic_params(),
                    );
                }
            }
        }
        for (g, m) in &self.typedefs {
            if Self::have_shrunk(&g.aliased, &m.aliased, g.generic_params()) {
                warn!(
                    "C++ bindings for {} (instantiated as {}) may be ill-formed: \
                    the aliased type contains zero-sized arguments or arrays",
                    g.path.name(),
                    m.path.name()
                );
            }
        }
    }

    fn warn_missing<'a>(
        generic_name: &str,
        monomorph_name: &str,
        generic: &'a [Field], // lifetime needed to make check_in_mono work
        monomorph: &[Field],
        params: &GenericParams,
    ) {
        let check_in_mono = |gf: &'a Field| {
            let mono = monomorph.iter().find(|mf| mf.name == gf.name);
            let Some(mf) = mono else {
                return Some(gf.name.as_str());
            };
            Self::have_shrunk(&gf.ty, &mf.ty, params).then(|| gf.name.as_str())
        };

        let invalid: Vec<&str> = generic.iter().filter_map(check_in_mono).collect();

        if !invalid.is_empty() {
            warn!(
                "C++ bindings for {} (instantiated as {}) may be ill-formed: \
                    field(s) {} are zero-sized or contain zero-sized arguments or arrays",
                generic_name,
                monomorph_name,
                invalid.join(", ")
            );
        }
    }

    // Checks whether specialization of `generic` into `monomorph` produced ill-formed C++ bindings.
    // In particular, we check for ZST arrays and function arguments.
    // This function follows the behavior of `GenericArgument::specialize` and `Type::specialize`,
    // so it must be updated if these functions introduce other kinds of ill-formed bindings.
    fn have_shrunk(generic: &Type, monomorph: &Type, params: &GenericParams) -> bool {
        use crate::bindgen::ir::{PrimitiveType, Type::*};
        match (generic, monomorph) {
            // `g` is one of the generic parameters, so `m` here is the argument that was substituted for it.
            // Just by itself, it might be correct; when leads to a dropped function argument or array of ZST,
            // we detect it a level higher, so we don't reach this case.
            // The case when it leads to a field being dropped is handled even before lost_structure is called.
            // If the substitution itself is another generic, it should be checked on its own.
            (Path(p), _) if params.iter().any(|gp| gp.name() == p.path()) => false,
            // `g` is a non-parameter path such as `gp<...>`: recurse into its generic arguments.
            (Path(gp), Path(mp)) => gp
                .generics()
                .iter()
                .zip(mp.generics())
                .any(|(g, m)| check_gen_arg(g, m, params)),
            (Ptr { ty: gt, .. }, Ptr { ty: mt, .. }) => Self::have_shrunk(gt, mt, params),
            (Array(ge, _), Array(me, _)) => Self::have_shrunk(ge, me, params),
            (
                FuncPtr {
                    ret: gr, args: ga, ..
                },
                FuncPtr {
                    ret: mr, args: ma, ..
                },
            ) => {
                ga.len() != ma.len() // Only recurse if arguments weren't dropped
                    || Self::have_shrunk(gr, mr, params)
                    || ga
                        .iter()
                        .zip(ma)
                        .any(|((_, g), (_, m))| Self::have_shrunk(g, m, params))
            }
            // An array of ZSTs is itself a ZST, and an enclosing type may turn it into void.
            // At the moment, it happens only with Type::Ptr and the return type of Type::FuncPtr.
            // The former is correct in C, but C++ keeps the template and ends up with an ill-formed `void (*)[N]`.
            // The latter is correct in C too (a ZST return becomes `void`), but the C++ template returns
            // an array, which is ill-formed for any `T`.
            (Array(_, _), Primitive(PrimitiveType::Void)) => true,
            (Primitive(_), _) => false,
            // Should be unreachable with the current `specialize`
            _ => {
                warn!("Unexpected specialization of {:?} into {:?}. Bindings might be ill-formed. Please report a cbindgen bug", generic, monomorph);
                true
            }
        }
    }

    pub fn mangle_path(&self, path: &GenericPath) -> Option<&Path> {
        self.replacements.get(path)
    }

    pub fn drain_opaques(&mut self) -> Vec<OpaqueItem> {
        mem::take(&mut self.opaques)
    }

    pub fn drain_structs(&mut self) -> Vec<Struct> {
        Self::drain_snd(&mut self.structs)
    }

    pub fn drain_unions(&mut self) -> Vec<Union> {
        Self::drain_snd(&mut self.unions)
    }

    pub fn drain_typedefs(&mut self) -> Vec<Typedef> {
        Self::drain_snd(&mut self.typedefs)
    }

    pub fn drain_enums(&mut self) -> Vec<Enum> {
        Self::drain_snd(&mut self.enums)
    }

    fn drain_snd<T, U>(v: &mut Vec<(T, U)>) -> Vec<U> {
        mem::take(v).into_iter().map(|(_, u)| u).collect()
    }
}

fn check_gen_arg(g: &GenericArgument, m: &GenericArgument, params: &GenericParams) -> bool {
    use crate::bindgen::ir::Type::*;
    match (g, m) {
        (GenericArgument::Type(g), GenericArgument::Type(m)) => {
            Monomorphs::have_shrunk(g, m, params)
        }
        // The generic argument became a ZST during specialization. This only happens when
        // (see `GenericArgument::specialize` and `Type::specialize`):
        // 1) `g` is a parameter mapped to a ZST. C++ gets `gp<void>`, whose validity
        //    depends on `gp`'s own instantiation which we don't check here.
        // 2) `g` is an array whose element type becomes a ZST (a parameter mapped to a ZST,
        //    or another such array). C++ gets `gp<void[N]>`, which is ill-formed.
        // Anything else means `specialize` changed; report it rather than panic.
        (GenericArgument::Type(g), GenericArgument::Zst(_)) => match g {
            Path(_) => false,
            Array(_, _) => true,
            _ => {
                warn!("Generic {:?} has been unexpectedly converted to a ZST. Bindings might be ill-formed. Please report a cbindgen bug", g);
                true
            }
        },
        _ => false,
    }
}
