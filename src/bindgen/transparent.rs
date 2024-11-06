use std::borrow::Cow;
use std::collections::HashMap;

use crate::bindgen::ir::{Constant, Static, Enum, Struct, Union, OpaqueItem, Typedef, Function, Item, ItemMap, GenericParams, GenericArgument, GenericParam, Field};
use crate::bindgen::library::Library;

/// Helper trait that makes it easier to work with `Cow` in iterators
pub trait IterCow: Iterator {
    /// Maps from `&T` to `Cow<'a, T>` using the provided closure. If the closure returns `Some`
    /// result, it is returned as `Cow::Owned`; otherwise, return the item as `Cow::Borrowed`.
    fn cow_map<'a, F, T>(self, f: F) -> impl Iterator<Item = Cow<'a, T>>
    where
        F: FnMut(&T) -> Option<T>,
        T: Clone + 'a,
        Self: Iterator<Item = &'a T>;

    /// True if any item is `Cow::Owned`
    fn any_owned<'i, 'a: 'i, T>(self) -> bool
    where
        T: Clone + 'a,
        Self: Iterator<Item = &'i Cow<'a, T>>;
}

// Blanket implementation for all iterators
impl<I: Iterator> IterCow for I {
    fn cow_map<'a, F, T>(self, mut f: F) -> impl Iterator<Item = Cow<'a, T>>
    where
        F: FnMut(&T) -> Option<T>,
        T: Clone + 'a,
        Self: Iterator<Item = &'a T>,
    {
        self.map(move |item| f(item).map(Cow::Owned).unwrap_or(Cow::Borrowed(item)))
    }

    fn any_owned<'i, 'a: 'i, T>(mut self) -> bool
    where
        T: Clone + 'a,
        Self: Iterator<Item = &'i Cow<'a, T>>,
    {
        self.any(|item| matches!(item, Cow::Owned(_)))
    }

}

/// Extension trait that compenates for `Cow::is_owned` being unstable
pub trait CowIsOwned {
    fn cow_is_owned(&self) -> bool;
}
impl<T: Clone> CowIsOwned for Cow<'_, T> {
    fn cow_is_owned(&self) -> bool {
        matches!(self, Cow::Owned(_))
    }
}

pub trait ResolveTransparentTypes: Sized {
    fn resolve_transparent_types(&self, library: &Library) -> Option<Self>;

    fn resolve_fields<'a>(library: &Library, fields: &'a Vec<Field>, params: &GenericParams, mut skip_first: bool) -> Cow<'a, Vec<Field>> {
        let new_fields: Vec<_> = fields.iter().cow_map(|f| {
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

        if new_fields.iter().any_owned() {
            Cow::Owned(new_fields.into_iter().map(Cow::into_owned).collect())
        } else {
            Cow::Borrowed(fields)
        }
    }

    fn resolve_generic_params<'a>(library: &Library, params: &'a GenericParams) -> Cow<'a, GenericParams> {
        // Resolve defaults in the generic params
        let new_params: Vec<_> = params.iter().cow_map(|param| match param.default()? {
            GenericArgument::Type(ty) => {
                // NOTE: Param defaults can reference other params
                let new_ty = ty.transparent_alias(library, params)?;
                let default = Some(GenericArgument::Type(new_ty));
                Some(GenericParam::new_type_param(param.name().name(), default))
            }
            _ => None,
        }).collect();

        if new_params.iter().any_owned() {
            Cow::Owned(GenericParams(new_params.into_iter().map(Cow::into_owned).collect()))
        } else {
            Cow::Borrowed(params)
        }
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
