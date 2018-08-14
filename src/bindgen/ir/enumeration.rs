/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use syn;

use bindgen::config::{Config, Language};
use bindgen::declarationtyperesolver::DeclarationTypeResolver;
use bindgen::dependencies::Dependencies;
use bindgen::ir::{
    AnnotationSet, Cfg, ConditionWrite, Documentation, GenericParams, GenericPath, Item,
    ItemContainer, Repr, ReprStyle, ReprType, Struct, ToCondition, Type,
};
use bindgen::library::Library;
use bindgen::rename::{IdentifierType, RenameRule};
use bindgen::utilities::find_first_some;
use bindgen::writer::{ListType, Source, SourceWriter};

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub discriminant: Option<u64>,
    pub body: Option<(String, Struct)>,
    pub documentation: Documentation,
}

impl EnumVariant {
    pub fn load(
        is_tagged: bool,
        variant: &syn::Variant,
        mod_cfg: &Option<Cfg>,
    ) -> Result<Self, String> {
        let discriminant = match &variant.discriminant {
            &Some((
                _,
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(ref lit),
                    ..
                }),
            )) => Some(lit.value()),
            &Some(_) => {
                return Err("Unsupported discriminant.".to_owned());
            }
            &None => None,
        };

        fn parse_fields(
            is_tagged: bool,
            fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
        ) -> Result<Vec<(String, Type, Documentation)>, String> {
            let mut res = Vec::new();

            if is_tagged {
                res.push((
                    "tag".to_string(),
                    Type::Path(GenericPath {
                        name: "Tag".to_string(),
                        generics: vec![],
                        ctype: None,
                    }),
                    Documentation::none(),
                ));
            }

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

            Ok(res)
        }

        let body = match variant.fields {
            syn::Fields::Unit => None,
            syn::Fields::Named(ref fields) => Some(Struct {
                name: format!("{}_Body", variant.ident),
                generic_params: GenericParams::default(),
                fields: parse_fields(is_tagged, &fields.named)?,
                is_tagged,
                tuple_struct: false,
                cfg: Cfg::append(mod_cfg, Cfg::load(&variant.attrs)),
                annotations: AnnotationSet::load(&variant.attrs)?,
                documentation: Documentation::none(),
            }),
            syn::Fields::Unnamed(ref fields) => Some(Struct {
                name: format!("{}_Body", variant.ident),
                generic_params: GenericParams::default(),
                fields: parse_fields(is_tagged, &fields.unnamed)?,
                is_tagged,
                tuple_struct: true,
                cfg: Cfg::append(mod_cfg, Cfg::load(&variant.attrs)),
                annotations: AnnotationSet::load(&variant.attrs)?,
                documentation: Documentation::none(),
            }),
        };

        Ok(EnumVariant {
            name: variant.ident.to_string(),
            discriminant,
            body: body.map(|body| {
                (
                    RenameRule::SnakeCase.apply_to_pascal_case(
                        &format!("{}", variant.ident),
                        IdentifierType::StructMember,
                    ),
                    body,
                )
            }),
            documentation: Documentation::load(&variant.attrs),
        })
    }

    fn add_dependencies(&self, library: &Library, out: &mut Dependencies) {
        if let &Some((_, ref item)) = &self.body {
            item.add_dependencies(library, out);
        }
    }

    fn resolve_declaration_types(&mut self, resolver: &DeclarationTypeResolver) {
        if let Some((_, ref mut ty)) = self.body {
            ty.resolve_declaration_types(resolver);
        }
    }
}

impl Source for EnumVariant {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        self.documentation.write(config, out);
        write!(out, "{}", self.name);
        if let Some(discriminant) = self.discriminant {
            write!(out, " = {}", discriminant);
        }
        out.write(",");
    }
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub repr: Repr,
    pub variants: Vec<EnumVariant>,
    pub tag: Option<String>,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}

impl Enum {
    fn can_derive_eq(&self) -> bool {
        if self.tag.is_none() {
            return false;
        }

        self.variants.iter().all(|variant| {
            variant.body.as_ref().map_or(true, |&(_, ref body)| {
                body.can_derive_eq()
            })
        })
    }

    pub fn load(item: &syn::ItemEnum, mod_cfg: &Option<Cfg>) -> Result<Enum, String> {
        let repr = Repr::load(&item.attrs)?;
        if repr == Repr::RUST {
            return Err("Enum not marked with a valid repr(prim) or repr(C).".to_owned());
        }

        let mut variants = Vec::new();
        let mut is_tagged = false;

        for variant in item.variants.iter() {
            let variant = EnumVariant::load(repr.style == ReprStyle::Rust, variant, mod_cfg)?;
            is_tagged = is_tagged || variant.body.is_some();
            variants.push(variant);
        }

        let annotations = AnnotationSet::load(&item.attrs)?;

        if let Some(names) = annotations.list("enum-trailing-values") {
            for name in names {
                variants.push(EnumVariant {
                    name,
                    discriminant: None,
                    body: None,
                    documentation: Documentation::none(),
                });
            }
        }

        Ok(Enum {
            name: item.ident.to_string(),
            repr,
            variants,
            tag: if is_tagged {
                Some("Tag".to_string())
            } else {
                None
            },
            cfg: Cfg::append(mod_cfg, Cfg::load(&item.attrs)),
            annotations,
            documentation: Documentation::load(&item.attrs),
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

    fn collect_declaration_types(&self, resolver: &mut DeclarationTypeResolver) {
        if self.tag.is_some() && self.repr.style == ReprStyle::C {
            resolver.add_struct(&self.name);
        } else if self.tag.is_some() && self.repr.style != ReprStyle::C {
            resolver.add_union(&self.name);
        } else if self.repr.style == ReprStyle::C {
            resolver.add_enum(&self.name);
        }
    }

    fn resolve_declaration_types(&mut self, resolver: &DeclarationTypeResolver) {
        for &mut ref mut var in &mut self.variants {
            var.resolve_declaration_types(resolver);
        }
    }

    fn rename_for_config(&mut self, config: &Config) {
        config.export.rename(&mut self.name);

        if config.language == Language::C && self.tag.is_some() {
            // it makes sense to always prefix Tag with type name in C
            let new_tag = format!("{}_Tag", self.name);
            if self.repr.style == ReprStyle::Rust {
                for variant in &mut self.variants {
                    if let Some((_, ref mut body)) = variant.body {
                        body.fields[0].1 = Type::Path(GenericPath {
                            name: new_tag.clone(),
                            generics: vec![],
                            ctype: None,
                        });
                    }
                }
            }
            self.tag = Some(new_tag);
        }

        for variant in &mut self.variants {
            if let Some((_, ref mut body)) = variant.body {
                body.rename_for_config(config);
            }
        }

        if config.enumeration.prefix_with_name
            || self.annotations.bool("prefix-with-name").unwrap_or(false)
        {
            for variant in &mut self.variants {
                variant.name = format!("{}_{}", self.name, variant.name);
                if let Some((_, ref mut body)) = variant.body {
                    body.name = format!("{}_{}", self.name, body.name);
                }
            }
        }

        let rules = [
            self.annotations.parse_atom::<RenameRule>("rename-all"),
            config.enumeration.rename_variants,
        ];

        if let Some(r) = find_first_some(&rules) {
            self.variants = self
                .variants
                .iter()
                .map(|variant| EnumVariant {
                    name: r.apply_to_pascal_case(&variant.name, IdentifierType::EnumVariant(self)),
                    discriminant: variant.discriminant.clone(),
                    body: variant.body.as_ref().map(|body| {
                        (
                            r.apply_to_snake_case(&body.0, IdentifierType::StructMember),
                            body.1.clone(),
                        )
                    }),
                    documentation: variant.documentation.clone(),
                }).collect();
        }
    }

    fn add_dependencies(&self, library: &Library, out: &mut Dependencies) {
        for variant in &self.variants {
            variant.add_dependencies(library, out);
        }
    }
}

impl Source for Enum {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        let size = self.repr.ty.map(|ty| match ty {
            ReprType::USize => "uintptr_t",
            ReprType::U32 => "uint32_t",
            ReprType::U16 => "uint16_t",
            ReprType::U8 => "uint8_t",
            ReprType::ISize => "intptr_t",
            ReprType::I32 => "int32_t",
            ReprType::I16 => "int16_t",
            ReprType::I8 => "int8_t",
        });

        let condition = (&self.cfg).to_condition(config);

        condition.write_before(config, out);

        self.documentation.write(config, out);

        let is_tagged = self.tag.is_some();
        let separate_tag = self.repr.style == ReprStyle::C;

        // If tagged, we need to emit a proper struct/union wrapper around our enum
        if is_tagged && config.language == Language::Cxx {
            out.write(if separate_tag { "struct " } else { "union " });
            write!(out, "{}", self.name);
            out.open_brace();
        }

        let enum_name = if let Some(ref tag) = self.tag {
            tag
        } else {
            &self.name
        };

        // Emit the actual enum
        if config.language == Language::C {
            if size.is_none() && config.style.generate_typedef() {
                out.write("typedef ");
            }

            out.write("enum");

            if !size.is_none() || config.style.generate_tag() {
                write!(out, " {}", enum_name);
            }
        } else {
            if let Some(prim) = size {
                write!(out, "enum class {} : {}", enum_name, prim);
            } else {
                write!(out, "enum class {}", enum_name);
            }
        }
        out.open_brace();
        for (i, variant) in self.variants.iter().enumerate() {
            if i != 0 {
                out.new_line()
            }
            variant.write(config, out);
        }
        if config.enumeration.add_sentinel(&self.annotations) {
            out.new_line();
            out.new_line();
            out.write("Sentinel /* this must be last for serialization purposes. */");
        }

        if config.language == Language::C && size.is_none() && config.style.generate_typedef() {
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
        // Done emitting the enum

        // If tagged, we need to emit structs for the cases and union them together
        if is_tagged {
            // Emit the cases for the structs
            for variant in &self.variants {
                if let Some((_, ref body)) = variant.body {
                    out.new_line();
                    out.new_line();

                    body.write(config, out);
                }
            }

            out.new_line();
            out.new_line();

            // Emit the actual union
            if config.language == Language::C {
                if config.style.generate_typedef() {
                    out.write("typedef ");
                }

                out.write(if separate_tag { "struct" } else { "union" });

                if config.style.generate_tag() {
                    write!(out, " {}", self.name);
                }

                out.open_brace();
            }

            // C++ allows accessing only common initial sequence of union
            // branches so we need to wrap tag into an anonymous struct
            let wrap_tag = config.language == Language::Cxx && !separate_tag;

            if wrap_tag {
                out.write("struct");
                out.open_brace();
            }

            if config.language == Language::C && !config.style.generate_typedef() {
                out.write("enum ");
            }

            write!(out, "{} tag;", enum_name);

            if wrap_tag {
                out.close_brace(true);
            }

            out.new_line();

            if separate_tag {
                out.write("union");
                out.open_brace();
            }

            for (i, &(ref field_name, ref body)) in self
                .variants
                .iter()
                .filter_map(|variant| variant.body.as_ref())
                .enumerate()
            {
                if i != 0 {
                    out.new_line();
                }
                if config.style.generate_typedef() {
                    write!(out, "{} {};", body.name, field_name);
                } else {
                    write!(out, "struct {} {};", body.name, field_name);
                }
            }

            if separate_tag {
                out.close_brace(true);
            }

            let skip_fields = if separate_tag { 0 } else { 1 };

            // Emit convenience methods
            if config.language == Language::Cxx
                && config.enumeration.derive_helper_methods(&self.annotations)
            {
                for variant in &self.variants {
                    out.new_line();
                    out.new_line();

                    let arg_renamer = |name: &str| {
                        config
                            .function
                            .rename_args
                            .as_ref()
                            .unwrap_or(&RenameRule::GeckoCase)
                            .apply_to_snake_case(name, IdentifierType::FunctionArg)
                    };

                    write!(out, "static {} {}(", self.name, variant.name);

                    if let Some((_, ref body)) = variant.body {
                        out.write_vertical_source_list(
                            &body
                                .fields
                                .iter()
                                .skip(skip_fields)
                                .map(|&(ref name, ref ty, _)| {
                                    // const-ref args to constructor
                                    (format!("const& {}", arg_renamer(name)), ty.clone())
                                }).collect(),
                            ListType::Join(","),
                        );
                    }

                    write!(out, ")");
                    out.open_brace();

                    write!(out, "{} result;", self.name);

                    if let Some((ref variant_name, ref body)) = variant.body {
                        for &(ref field_name, ..) in body.fields.iter().skip(skip_fields) {
                            out.new_line();
                            write!(
                                out,
                                "result.{}.{} = {};",
                                variant_name,
                                field_name,
                                arg_renamer(field_name)
                            );
                        }
                    }

                    out.new_line();
                    write!(out, "result.tag = {}::{};", enum_name, variant.name);
                    out.new_line();
                    write!(out, "return result;");
                    out.close_brace(false);
                }

                for variant in &self.variants {
                    out.new_line();
                    out.new_line();

                    // FIXME: create a config for method case
                    write!(out, "bool Is{}() const", variant.name);
                    out.open_brace();
                    write!(out, "return tag == {}::{};", enum_name, variant.name);
                    out.close_brace(false);
                }
            }

            if config.language == Language::Cxx &&
                self.can_derive_eq() &&
                config.structure.derive_eq(&self.annotations)
            {
                out.new_line();
                out.new_line();
                write!(out, "bool operator==(const {}& other) const", self.name);
                out.open_brace();
                write!(out, "if (tag != other.tag)");
                out.open_brace();
                write!(out, "return false;");
                out.close_brace(false);
                out.new_line();
                write!(out, "switch (tag)");
                out.open_brace();
                for variant in &self.variants {
                    if let Some((ref variant_name, _)) = variant.body {
                        write!(
                            out,
                            "case {}::{}: return {} == other.{};",
                            self.tag.as_ref().unwrap(),
                            variant.name,
                            variant_name,
                            variant_name
                        );
                        out.new_line();
                    }
                }
                write!(out, "default: return true;");
                out.close_brace(false);
                out.close_brace(false);

                if config.structure.derive_neq(&self.annotations) {
                    out.new_line();
                    out.new_line();
                    write!(out, "bool operator!=(const {}& other) const", self.name);
                    out.open_brace();
                    write!(out, "return !(*this == other);");
                    out.close_brace(false);
                }
            }

            if config.language == Language::C {
                if config.style.generate_typedef() {
                    out.close_brace(false);
                    write!(out, " {};", self.name);
                } else {
                    out.close_brace(true);
                }
            } else {
                out.close_brace(true);
            }
        }

        condition.write_after(config, out);
    }
}
