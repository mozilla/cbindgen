/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use syn;

use bindgen::config::{Config, Language};
use bindgen::rename::*;
use bindgen::utilities::*;
use bindgen::writer::*;
use bindgen::ir::{AnnotationSet, Documentation};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Repr {
    None,
    C,
    U8,
    U16,
    U32,
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub repr: Repr,
    pub annotations: AnnotationSet,
    pub values: Vec<(String, u64, Documentation)>,
    pub documentation: Documentation,
}

impl Enum {
    pub fn load(name: String,
                repr: Repr,
                annotations: AnnotationSet,
                variants: &Vec<syn::Variant>,
                doc: String) -> Result<Enum, String>
    {
        if repr != Repr::U32 &&
           repr != Repr::U16 &&
           repr != Repr::U8 {
            return if repr == Repr::C {
                Err(format!("repr(C) is not FFI safe for enums"))
            } else {
                Err(format!("enum not marked with a repr(u32) or repr(u16) or repr(u8)"))
            };
        }

        let mut values = Vec::new();
        let mut current = 0;

        for variant in variants {
            match variant.data {
                syn::VariantData::Unit => {
                    match variant.discriminant {
                        Some(syn::ConstExpr::Lit(syn::Lit::Int(i, _))) => {
                            current = i;
                        }
                        Some(_) => {
                            return Err(format!("unsupported discriminant"));
                        }
                        None => { /* okay, we just use current */ }
                    }

                    values.push((variant.ident.to_string(), current, Documentation::load(variant.get_doc_attr())));
                    current = current + 1;
                }
                _ => {
                    return Err(format!("unsupported variant"));
                }
            }
        }

        if let Some(variants) = annotations.list("enum-trailing-values") {
            for variant in variants {
                values.push((variant, current, Documentation::none()));
                current = current + 1;
            }
        }

        Ok(Enum {
            name: name,
            repr: repr,
            annotations: annotations,
            values: values,
            documentation: Documentation::load(doc),
        })
    }

    pub fn rename_values(&mut self, config: &Config) {
        if config.enumeration.prefix_with_name ||
           self.annotations.bool("prefix-with-name").unwrap_or(false)
        {
            let old = ::std::mem::replace(&mut self.values, Vec::new());
            for (name, value, doc) in old {
                self.values.push((format!("{}_{}", self.name, name), value, doc));
            }
        }

        let rules = [self.annotations.parse_atom::<RenameRule>("rename-all"),
                     config.enumeration.rename_variants];

        if let Some(r) = find_first_some(&rules) {
            self.values = self.values.iter()
                                     .map(|x| (r.apply_to_pascal_case(&x.0,
                                                                      IdentifierType::EnumVariant(self)),
                                               x.1.clone(),
                                               x.2.clone()))
                                     .collect();
        }
    }
}

impl Source for Enum {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        let size = match self.repr {
            Repr::U32 => "uint32_t",
            Repr::U16 => "uint16_t",
            Repr::U8 => "uint8_t",
            _ => unreachable!(),
        };
        self.documentation.write(config, out);
        if config.language == Language::C {
            out.write(&format!("enum {}", self.name));
        } else {
            out.write(&format!("enum class {} : {}", self.name, size));
        }
        out.open_brace();
        for (i, value) in self.values.iter().enumerate() {
            if i != 0 {
                out.new_line()
            }
            value.2.write(config, out);
            out.write(&format!("{} = {},", value.0, value.1));
        }
        if config.enumeration.add_sentinel(&self.annotations) {
            out.new_line();
            out.new_line();
            out.write("Sentinel /* this must be last for serialization purposes. */");
        }
        out.close_brace(true);

        if config.language == Language::C {
            out.new_line();
            out.write(&format!("typedef {} {};", size, self.name));
        }
    }
}
