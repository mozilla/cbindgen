use syn::ext::IdentExt;

use crate::bindgen::ir::{AnnotationSet, Cfg, Zst};
use crate::bindgen::ir::{Documentation, Path, Type};

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub ty: Type,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}

impl Field {
    pub fn from_name_and_type(name: String, ty: Type) -> Field {
        Field {
            name,
            ty,
            cfg: None,
            annotations: AnnotationSet::new(),
            documentation: Documentation::none(),
        }
    }

    pub fn load(field: &syn::Field, self_path: &Path) -> Result<Option<Field>, String> {
        // So far, the only reason to omit a field is because its type is 1-ZST.
        // Both structs and enums will drop it.
        // Therefore, we don't need to propagate the reason for omitting, and Option suffices.
        // TODO: is it sufficient in general?
        let mut ty = match Type::load(&field.ty)? {
            Ok(ty) => ty,
            // Right now we account only for ZSTs with 1-alignment,
            // so it's safe to skip them
            Err(Zst::Zst1) => return Ok(None),
        };
        ty.replace_self_with(self_path);
        Ok(Some(Field {
            name: field
                .ident
                .as_ref()
                .ok_or_else(|| "field is missing identifier".to_string())?
                .unraw()
                .to_string(),
            ty,
            cfg: Cfg::load(&field.attrs),
            annotations: AnnotationSet::load(&field.attrs)?,
            documentation: Documentation::load(&field.attrs),
        }))
    }
}
