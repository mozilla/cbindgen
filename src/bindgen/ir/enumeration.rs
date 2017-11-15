/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use syn;

use bindgen::config::{Config, Language};
use bindgen::ir::{AnnotationSet, Cfg, CfgWrite, Documentation, GenericParams, GenericPath, Item,
                  ItemContainer, Repr, Struct, Type};
use bindgen::rename::{IdentifierType, RenameRule};
use bindgen::utilities::find_first_some;
use bindgen::writer::{Source, SourceWriter};

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub repr: Repr,
    pub values: Vec<(String, u64, Option<Struct>, Documentation)>,
    pub tag: Option<String>,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}

impl Enum {
    pub fn load(
        name: String,
        variants: &Vec<syn::Variant>,
        attrs: &Vec<syn::Attribute>,
        mod_cfg: &Option<Cfg>,
    ) -> Result<Enum, String> {
        let repr = Repr::load(attrs);

        if repr != Repr::C && repr != Repr::USize && repr != Repr::U32 && repr != Repr::U16
            && repr != Repr::U8 && repr != Repr::ISize && repr != Repr::I32
            && repr != Repr::I16 && repr != Repr::I8
        {
            return Err("Enum not marked with a valid repr(prim) or repr(C).".to_owned());
        }

        let mut values = Vec::new();
        let mut current = 0;
        let mut is_tagged = false;

        for variant in variants {
            match variant.discriminant {
                Some(syn::ConstExpr::Lit(syn::Lit::Int(i, _))) => {
                    current = i;
                }
                Some(_) => {
                    return Err("Unsupported discriminant.".to_owned());
                }
                None => { /* okay, we just use current */ }
            };
            let body = match variant.data {
                syn::VariantData::Unit => None,
                syn::VariantData::Struct(ref fields) | syn::VariantData::Tuple(ref fields) => {
                    is_tagged = true;
                    Some(Struct {
                        name: format!("{}_Body", variant.ident),
                        generic_params: GenericParams::default(),
                        fields: {
                            let mut res = vec![
                                (
                                    "tag".to_string(),
                                    Type::Path(GenericPath {
                                        name: "Tag".to_string(),
                                        generics: vec![],
                                    }),
                                    Documentation::none(),
                                ),
                            ];

                            for (i, field) in fields.iter().enumerate() {
                                if let Some(ty) = Type::load(&field.ty)? {
                                    res.push((
                                        match field.ident {
                                            Some(ref ident) => ident.to_string(),
                                            None => i.to_string(),
                                        },
                                        ty,
                                        Documentation::load(&field.attrs),
                                    ));
                                }
                            }

                            res
                        },
                        is_variant: true,
                        tuple_struct: match variant.data {
                            syn::VariantData::Tuple(_) => true,
                            _ => false,
                        },
                        cfg: Cfg::append(mod_cfg, Cfg::load(attrs)),
                        annotations: AnnotationSet::load(attrs)?,
                        documentation: Documentation::none(),
                    })
                }
            };
            values.push((
                variant.ident.to_string(),
                current,
                body,
                Documentation::load(&variant.attrs),
            ));
            current = current + 1;
        }

        let annotations = AnnotationSet::load(attrs)?;

        if let Some(variants) = annotations.list("enum-trailing-values") {
            for variant in variants {
                values.push((variant, current, None, Documentation::none()));
                current = current + 1;
            }
        }

        Ok(Enum {
            name: name,
            repr: repr,
            values: values,
            tag: if is_tagged {
                Some("Tag".to_string())
            } else {
                None
            },
            cfg: Cfg::append(mod_cfg, Cfg::load(attrs)),
            annotations: annotations,
            documentation: Documentation::load(attrs),
        })
    }
}

impl Item for Enum {
    fn name(&self) -> &str {
        &self.name
    }

    fn cfg(&self) -> &Option<Cfg> {
        &self.cfg
    }

    fn annotations(&self) -> &AnnotationSet {
        &self.annotations
    }

    fn annotations_mut(&mut self) -> &mut AnnotationSet {
        &mut self.annotations
    }

    fn container(&self) -> ItemContainer {
        ItemContainer::Enum(self.clone())
    }

    fn rename_for_config(&mut self, config: &Config) {
        if config.language == Language::C && self.tag.is_some() {
            // it makes sense to always prefix Tag with type name in C
            let new_tag = format!("{}_Tag", self.name);
            for value in &mut self.values {
                if let Some(ref mut body) = value.2 {
                    body.fields[0].1 = Type::Path(GenericPath {
                        name: new_tag.clone(),
                        generics: vec![],
                    });
                }
            }
            self.tag = Some(new_tag);
        }

        for value in &mut self.values {
            if let Some(ref mut body) = value.2 {
                body.rename_for_config(config);
            }
        }

        if config.enumeration.prefix_with_name
            || self.annotations.bool("prefix-with-name").unwrap_or(false)
        {
            for value in &mut self.values {
                value.0 = format!("{}_{}", self.name, value.0);
                if let Some(ref mut body) = value.2 {
                    body.name = format!("{}_{}", self.name, body.name);
                }
            }
        }

        let rules = [
            self.annotations.parse_atom::<RenameRule>("rename-all"),
            config.enumeration.rename_variants,
        ];

        if let Some(r) = find_first_some(&rules) {
            self.values = self.values
                .iter()
                .map(|x| {
                    (
                        r.apply_to_pascal_case(&x.0, IdentifierType::EnumVariant(self)),
                        x.1.clone(),
                        x.2.clone(),
                        x.3.clone(),
                    )
                })
                .collect();
        }
    }
}

impl Source for Enum {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        self.cfg.write_before(config, out);

        self.documentation.write(config, out);

        let is_tagged = self.tag.is_some();

        if is_tagged && config.language == Language::Cxx {
            write!(out, "union {}", self.name);
            out.open_brace();
        }

        let enum_name = if let Some(ref tag) = self.tag {
            tag
        } else {
            &self.name
        };

        let size = match self.repr {
            Repr::C => None,
            Repr::USize => Some("uintptr_t"),
            Repr::U32 => Some("uint32_t"),
            Repr::U16 => Some("uint16_t"),
            Repr::U8 => Some("uint8_t"),
            Repr::ISize => Some("intptr_t"),
            Repr::I32 => Some("int32_t"),
            Repr::I16 => Some("int16_t"),
            Repr::I8 => Some("int8_t"),
            _ => unreachable!(),
        };

        if config.language == Language::C {
            if size.is_none() {
                out.write("typedef enum");
            } else {
                write!(out, "enum {}", enum_name);
            }
        } else {
            if let Some(prim) = size {
                write!(out, "enum class {} : {}", enum_name, prim);
            } else {
                write!(out, "enum class {}", enum_name);
            }
        }
        out.open_brace();
        for (i, value) in self.values.iter().enumerate() {
            if i != 0 {
                out.new_line()
            }
            value.3.write(config, out);
            write!(out, "{} = {},", value.0, value.1);
        }
        if config.enumeration.add_sentinel(&self.annotations) {
            out.new_line();
            out.new_line();
            out.write("Sentinel /* this must be last for serialization purposes. */");
        }

        if config.language == Language::C && size.is_none() {
            out.close_brace(false);
            write!(out, " {};", enum_name);
        } else {
            out.close_brace(true);
        }

        if config.language == Language::C {
            if let Some(prim) = size {
                out.new_line();
                write!(out, "typedef {} {};", prim, enum_name);
            }
        }

        if is_tagged {
            for value in &self.values {
                if let Some(ref body) = value.2 {
                    out.new_line();
                    out.new_line();

                    body.write(config, out);
                }
            }

            out.new_line();
            out.new_line();

            if config.language == Language::C {
                out.write("typedef union");
                out.open_brace();
            }

            write!(out, "{} tag;", enum_name);

            for value in &self.values {
                if let Some(ref body) = value.2 {
                    out.new_line();
                    write!(out, "{} {};", body.name, value.0);
                }
            }

            if config.language == Language::C {
                out.close_brace(false);
                write!(out, " {};", self.name);
            } else {
                out.close_brace(true);
            }
        }

        self.cfg.write_after(config, out);
    }
}
