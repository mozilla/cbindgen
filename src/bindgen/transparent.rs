use std::collections::HashMap;

use crate::bindgen::ir::{Constant, Static, Enum, Struct, Union, OpaqueItem, Typedef, Function, Item, ItemMap};
use crate::bindgen::library::Library;

pub trait ResolveTransparentTypes: Sized {
    fn resolve_transparent_types(&self, library: &Library) -> Option<Self>;
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
