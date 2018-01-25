use std::ops::Deref;
use std::io::Write;

use syn;

use bindgen::config::{Config, Language};
use bindgen::writer::{Source, SourceWriter};

#[derive(Default, Debug, Clone)]
pub struct GenericParams(pub Vec<String>);

impl GenericParams {
    pub fn new(generics: &syn::Generics) -> Self {
        GenericParams(
            generics
                .params
                .iter()
                .filter_map(|x| match x {
                    &syn::GenericParam::Type(syn::TypeParam { ref ident, .. }) => Some(ident.to_string()),
                    _ => None,
                })
                .collect::<Vec<_>>(),
        )
    }
}

impl Deref for GenericParams {
    type Target = [String];

    fn deref(&self) -> &[String] {
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
