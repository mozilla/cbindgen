use std::collections::HashMap;

use crate::bindgen::ir::{Constant, Static, Enum, Struct, Union, OpaqueItem, Typedef, Function, Item, ItemMap, GenericParams, GenericArgument, GenericParam, Field};
use crate::bindgen::library::Library;

pub trait ResolveTransparentTypes: Sized {
    fn resolve_transparent_types(&self, library: &Library) -> Option<Self>;

    fn resolve_fields(library: &Library, fields: &[Field], params: &GenericParams, mut skip_first: bool) -> Option<Vec<Field>> {
        let new_fields: Vec<_> = fields.iter().map(|f| {
            // Ignore the inline Tag field, if any (it's always first)
            if skip_first {
                skip_first = false;
                None
            } else {
                Some(Field {
                    ty: f.ty.transparent_alias(library, params)?,
                    ..f.clone()
                })
            }
        }).collect();

        if new_fields.iter().all(Option::is_none) {
            return None;
        }
        Some(new_fields.into_iter().zip(fields).map(|(new_field, field)| {
            new_field.unwrap_or_else(|| field.clone())
        }).collect())
    }

    fn resolve_generic_params(library: &Library, params: &GenericParams) -> Option<GenericParams> {
        // Resolve defaults in the generic params
        let new_params: Vec<_> = params.iter().map(|param| {
            match param.default()? {
                GenericArgument::Type(ty) => {
                    // NOTE: Param defaults can reference other params
                    let new_ty = ty.transparent_alias(library, params)?;
                    let default = Some(GenericArgument::Type(new_ty));
                    Some(GenericParam::new_type_param(param.name().name(), default))
                }
                _ => None,
            }
        }).collect();

        new_params.iter().any(Option::is_some).then(|| {
            let params = new_params.into_iter().zip(&params.0).map(|(new_param, param)| {
                new_param.unwrap_or_else(|| param.clone())
            });
            GenericParams(params.collect())
        })
    }
}

pub type ResolvedItems<T> = HashMap<usize, T>;

/// An indirection that allows to generalize the two-stage process of resolving transparent types.
#[derive(Default)]
pub struct TransparentTypeResolver {
    pub constants: ResolvedItems<Constant>,
    pub globals: ResolvedItems<Static>,
    pub enums: ResolvedItems<Enum>,
    pub structs: ResolvedItems<Struct>,
    pub unions: ResolvedItems<Union>,
    pub opaque_items: ResolvedItems<OpaqueItem>,
    pub typedefs: ResolvedItems<Typedef>,
    pub functions: ResolvedItems<Function>,
}

impl TransparentTypeResolver {
    fn resolve_item<T: ResolveTransparentTypes>(item: &T, i: usize, resolved: &mut ResolvedItems<T>, library: &Library) {
        if let Some(alias) = item.resolve_transparent_types(library) {
            resolved.insert(i, alias);
        }
    }

    // Functions do not impl Item
    pub fn resolve_functions(&mut self, library: &Library, items: &Vec<Function>) {
        for (i, item) in items.into_iter().enumerate() {
            Self::resolve_item(item, i, &mut self.functions, library);
        }
    }

    pub fn resolve_items<T: ResolveTransparentTypes + Item + Clone>(resolved: &mut ResolvedItems<T>, library: &Library, items: &ItemMap<T>) {
        let mut i = 0;
        items.for_all_items(|item| {
            Self::resolve_item(item, i, resolved, library);
            i += 1;
        });
    }

    fn install_item<T: ResolveTransparentTypes>(item: &mut T, i: usize, resolved: &mut ResolvedItems<T>) {
        if let Some(alias) = resolved.remove(&i) {
            *item = alias;
        }
    }

    // Functions do not impl Item
    pub fn install_functions(&mut self, items: &mut Vec<Function>) {
        for (i, item) in items.into_iter().enumerate() {
            Self::install_item(item, i, &mut self.functions);
        }
    }

    pub fn install_items<T: ResolveTransparentTypes + Item + Clone>(resolved: &mut ResolvedItems<T>, items: &mut ItemMap<T>) {
        let mut i = 0;
        items.for_all_items_mut(|item| {
            Self::install_item(item, i, resolved);
            i += 1;
        });
    }
}
