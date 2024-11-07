use std::borrow::Cow;
use std::collections::HashMap;

use crate::bindgen::ir::{
    Field, Function, GenericArgument, GenericParam, GenericParams, Item, ItemContainer, ItemMap,
    Path, Type,
};
use crate::bindgen::library::Library;

/// Helper trait that makes it easier to work with `Cow` in iterators
pub trait IterCow: Iterator {
    /// Maps from `&T` to `Cow<'a, T>` using the provided function. If the function returns `Some`
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
        self.map(move |item| f(item).map_or_else(|| Cow::Borrowed(item), Cow::Owned))
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
    /// Attempts to resolve transparent aliases for all `Type` instances in this item. This includes
    /// directly embedded types as well as generic parameter defaults, if any.
    fn resolve_transparent_types(&self, library: &Library) -> Option<Self>;

    /// Attempts to resolve transparent aliases for the types of a slice of fields. A `Cow::Owned`
    /// result means at least one field was modified as a result; otherwise, the original fields are
    /// returned as a `Cow::Borrowed` result.
    fn resolve_fields<'a>(
        library: &Library,
        fields: &'a Vec<Field>,
        params: &GenericParams,
        mut skip_first: bool,
    ) -> Cow<'a, Vec<Field>> {
        let new_fields: Vec<_> = fields
            .iter()
            .cow_map(|f| {
                // Ignore the inline Tag field, if any (it's always first, when present at all)
                if skip_first {
                    skip_first = false;
                    None
                } else {
                    Some(Field {
                        ty: f.ty.transparent_alias(library, params)?,
                        ..f.clone()
                    })
                }
            })
            .collect();

        if new_fields.iter().any_owned() {
            Cow::Owned(new_fields.into_iter().map(Cow::into_owned).collect())
        } else {
            Cow::Borrowed(fields)
        }
    }

    /// Attempts to resolve transparent aliases for the types of default values in a generic
    /// parameter list. A `Cow::Owned` return value means at least one field was modified as a
    /// result; otherwise, the original params are returned as a `Cow::Borrowed` value.
    fn resolve_generic_params<'a>(
        library: &Library,
        params: &'a GenericParams,
    ) -> Cow<'a, GenericParams> {
        let new_params: Vec<_> = params
            .iter()
            .cow_map(|param| match param.default() {
                Some(GenericArgument::Type(ty)) => {
                    // NOTE: Param defaults can reference other params, so forward the param list to
                    // the type resolution to allow for proper substitutions to be made.
                    let new_ty = ty.transparent_alias(library, params)?;
                    let default = Some(GenericArgument::Type(new_ty));
                    Some(GenericParam::new_type_param(param.name().name(), default))
                }
                _ => None,
            })
            .collect();

        if new_params.iter().any_owned() {
            Cow::Owned(GenericParams(
                new_params.into_iter().map(Cow::into_owned).collect(),
            ))
        } else {
            Cow::Borrowed(params)
        }
    }
}

pub type ResolvedItems<T> = HashMap<usize, T>;

/// An indirection that allows to generalize the two-stage process of resolving transparent
/// types. We first call a `resolve_XXX` function to resolve transparent aliases and store the
/// results in a hashmap, using a borrowed reference to the `Library` to resolve the types. In the
/// second step, we call an `install_XXX` function to update a mutable reference to the `Library`.
pub struct TransparentTypeResolver;

impl TransparentTypeResolver {
    /// Attempts to resolve a path as a transparent alias. Even if this function returns None, the
    /// caller has also resolved the generics and may need to return them.
    pub fn transparent_alias_for_path(
        path: &Path,
        generics: &[GenericArgument],
        library: &Library,
        params: &GenericParams,
    ) -> Option<Type> {
        let Some(items) = library.get_items(path) else {
            warn!("Unknown type {path:?}");
            return None;
        };
        let mut items = items.into_iter();
        let item = items.next()?;
        if let Some(other_item) = items.next() {
            warn!("Found multiple resolved types for {path:?}: {item:?} and. {other_item:?}");
            return None;
        }

        // Only some item types can ever be transparent -- handle them directly (not via `Item`)
        let resolved_type = match item {
            ItemContainer::Typedef(t) => t.transparent_alias(generics)?,
            ItemContainer::Struct(s) => s.transparent_alias(generics)?,
            ItemContainer::OpaqueItem(o) => o.transparent_alias(generics)?,
            _ => return None,
        };

        // The resolved type may itself be transparent, so recurse on it
        resolved_type
            .transparent_alias(library, params)
            .or(Some(resolved_type))
    }

    pub fn resolve_items<T>(&self, library: &Library, items: &ItemMap<T>) -> ResolvedItems<T>
    where
        T: ResolveTransparentTypes + Item + Clone,
    {
        let mut resolved = Default::default();
        let mut i = 0;
        items.for_all_items(|item| {
            Self::resolve_item(item, i, &mut resolved, library);
            i += 1;
        });
        resolved
    }
    pub fn install_items<T>(&self, mut resolved: ResolvedItems<T>, items: &mut ItemMap<T>)
    where
        T: ResolveTransparentTypes + Item + Clone,
    {
        let mut i = 0;
        items.for_all_items_mut(|item| {
            Self::install_item(item, i, &mut resolved);
            i += 1;
        });
    }

    // Functions do not impl Item
    pub fn resolve_functions(
        &self,
        library: &Library,
        items: &[Function],
    ) -> ResolvedItems<Function> {
        let mut functions = Default::default();
        for (i, item) in items.iter().enumerate() {
            Self::resolve_item(item, i, &mut functions, library);
        }
        functions
    }
    pub fn install_functions(
        &self,
        mut functions: ResolvedItems<Function>,
        items: &mut [Function],
    ) {
        for (i, item) in items.iter_mut().enumerate() {
            Self::install_item(item, i, &mut functions);
        }
    }

    fn resolve_item<T>(item: &T, i: usize, resolved: &mut ResolvedItems<T>, library: &Library)
    where
        T: ResolveTransparentTypes,
    {
        if let Some(alias) = item.resolve_transparent_types(library) {
            resolved.insert(i, alias);
        }
    }
    fn install_item<T>(item: &mut T, i: usize, resolved: &mut ResolvedItems<T>)
    where
        T: ResolveTransparentTypes,
    {
        if let Some(alias) = resolved.remove(&i) {
            *item = alias;
        }
    }
}
