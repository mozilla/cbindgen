use std::io::Write;

use crate::bindgen::cdecl;
use crate::bindgen::config::Config;
use crate::bindgen::ir::{AnnotationSet, Documentation, Path, Type};
use crate::bindgen::writer::{Source, SourceWriter};

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub ty: Type,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}

impl Field {
    pub fn from_name_and_type(name: String, ty: Type) -> Field {
        Field {
            name,
            ty,
            annotations: AnnotationSet::new(),
            documentation: Documentation::none(),
        }
    }

    pub fn load(field: &syn::Field, self_path: &Path) -> Result<Option<Field>, String> {
        Ok(if let Some(mut ty) = Type::load(&field.ty)? {
            ty.replace_self_with(self_path);
            Some(Field {
                name: field
                    .ident
                    .as_ref()
                    .ok_or_else(|| "field is missing identifier".to_string())?
                    .to_string(),
                ty,
                annotations: AnnotationSet::load(&field.attrs)?,
                documentation: Documentation::load(&field.attrs),
            })
        } else {
            None
        })
    }
}

impl Source for Field {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        self.documentation.write(config, out);
        cdecl::write_field(out, &self.ty, &self.name, config);
    }
}
