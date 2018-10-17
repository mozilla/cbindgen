use std::io::Write;
use std::ops::Deref;

use syn;

use bindgen::config::{Config, Language};
use bindgen::declarationtyperesolver::{DeclarationType, DeclarationTypeResolver};
use bindgen::ir::{Path, Type};
use bindgen::utilities::IterHelpers;
use bindgen::writer::{Source, SourceWriter};

#[derive(Default, Debug, Clone)]
pub struct GenericParams(pub Vec<Path>);

impl GenericParams {
    pub fn new(generics: &syn::Generics) -> Self {
        GenericParams(
            generics
                .params
                .iter()
                .filter_map(|x| match x {
                    &syn::GenericParam::Type(syn::TypeParam { ref ident, .. }) => {
                        Some(Path::new(ident.to_string()))
                    }
                    _ => None,
                })
                .collect(),
        )
    }
}

impl Deref for GenericParams {
    type Target = [Path];

    fn deref(&self) -> &[Path] {
        &self.0
    }
}

impl Source for GenericParams {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        if !self.0.is_empty() {
            if config.language == Language::Cxx {
                out.write("template<");
                for (i, item) in self.0.iter().enumerate() {
                    if i != 0 {
                        out.write(", ");
                    }
                    write!(out, "typename {}", item);
                }
                out.write(">");
                out.new_line();
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Generic {
    path: Path,
    export_name: String,
    generics: Vec<Type>,
    ctype: Option<DeclarationType>,
}

impl Generic {
    pub fn new(path: Path, generics: Vec<Type>) -> Generic {
        let export_name = path.name().to_owned();
        Self {
            path: path,
            export_name: export_name,
            generics: generics,
            ctype: None,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn generics(&self) -> &[Type] {
        &self.generics
    }

    pub fn ctype(&self) -> Option<&DeclarationType> {
        self.ctype.as_ref()
    }

    pub fn name(&self) -> &str {
        self.path.name()
    }

    pub fn export_name(&self) -> &str {
        &self.export_name
    }

    pub fn rename_for_config(&mut self, config: &Config, generic_params: &GenericParams) {
        for generic in &mut self.generics {
            generic.rename_for_config(config, generic_params);
        }
        config.export.rename(&mut self.export_name);
    }

    pub fn resolve_declaration_types(&mut self, resolver: &DeclarationTypeResolver) {
        self.ctype = resolver.type_for(&self.path);
    }

    pub fn load(path: &syn::Path) -> Result<Generic, String> {
        assert!(
            path.segments.len() > 0,
            "{:?} doesn't have any segments",
            path
        );
        let last_segment_token = path.segments.last().unwrap();
        let last_segment = last_segment_token.value();
        let name = last_segment.ident.to_string();

        let path = Path::new(name);
        let phantom_data_path = Path::new("PhantomData");
        if path == phantom_data_path {
            return Ok(Generic::new(path, Vec::new()));
        }

        let generics = match &last_segment.arguments {
            &syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                ref args,
                ..
            }) => args.iter().try_skip_map(|x| match *x {
                &syn::GenericArgument::Type(ref x) => Type::load(x),
                _ => Err(String::new()),
            })?,
            &syn::PathArguments::Parenthesized(_) => {
                return Err("Path contains parentheses.".to_owned());
            }
            _ => Vec::new(),
        };

        Ok(Generic::new(path, generics))
    }
}
