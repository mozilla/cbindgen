/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use crate::bindgen::config::{Config, Language};
use crate::bindgen::declarationtyperesolver::DeclarationTypeResolver;
use crate::bindgen::dependencies::Dependencies;
use crate::bindgen::ir::{
    AnnotationSet, AnnotationValue, Cfg, ConditionWrite, Documentation, GenericParams, GenericPath,
    Item, ItemContainer, Path, Repr, ReprStyle, ReprType, Struct, ToCondition, Type,
};
use crate::bindgen::library::Library;
use crate::bindgen::mangle;
use crate::bindgen::monomorph::Monomorphs;
use crate::bindgen::rename::{IdentifierType, RenameRule};
use crate::bindgen::reserved;
use crate::bindgen::writer::{ListType, Source, SourceWriter};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum VariantBody {
    Empty(AnnotationSet),
    Body {
        /// The variant field / export name.
        name: String,
        /// The struct with all the items.
        body: Struct,
    },
}

impl VariantBody {
    fn empty() -> Self {
        Self::Empty(AnnotationSet::new())
    }

    fn annotations(&self) -> &AnnotationSet {
        match *self {
            Self::Empty(ref anno) => anno,
            Self::Body { ref body, .. } => &body.annotations,
        }
    }

    fn is_empty(&self) -> bool {
        match *self {
            Self::Empty(..) => true,
            Self::Body { .. } => false,
        }
    }

    fn specialize(
        &self,
        generic_values: &[Type],
        mappings: &[(&Path, &Type)],
        config: &Config,
    ) -> Self {
        match *self {
            Self::Empty(ref annos) => Self::Empty(annos.clone()),
            Self::Body { ref name, ref body } => Self::Body {
                name: name.clone(),
                body: body.specialize(generic_values, mappings, config),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub export_name: String,
    pub discriminant: Option<i64>,
    pub body: VariantBody,
    pub cfg: Option<Cfg>,
    pub documentation: Documentation,
}

fn value_from_expr(val: &syn::Expr) -> Option<i64> {
    match *val {
        syn::Expr::Lit(ref lit) => match lit.lit {
            syn::Lit::Int(ref lit) => lit.base10_parse::<i64>().ok(),
            _ => None,
        },
        syn::Expr::Unary(ref unary) => {
            let v = value_from_expr(&unary.expr)?;
            match unary.op {
                syn::UnOp::Deref(..) => None,
                syn::UnOp::Neg(..) => v.checked_mul(-1),
                syn::UnOp::Not(..) => v.checked_neg(),
            }
        }
        _ => None,
    }
}

impl EnumVariant {
    fn load(
        is_tagged: bool,
        variant: &syn::Variant,
        generic_params: GenericParams,
        mod_cfg: Option<&Cfg>,
        self_path: &Path,
        enum_annotations: &AnnotationSet,
    ) -> Result<Self, String> {
        let discriminant = match variant.discriminant {
            Some((_, ref expr)) => match value_from_expr(expr) {
                Some(v) => Some(v),
                None => return Err(format!("Unsupported discriminant {:?}.", expr)),
            },
            None => None,
        };

        fn parse_fields(
            is_tagged: bool,
            fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
            self_path: &Path,
        ) -> Result<Vec<(String, Type, Documentation)>, String> {
            let mut res = Vec::new();

            if is_tagged {
                res.push((
                    "tag".to_string(),
                    Type::Path(GenericPath::new(Path::new("Tag"), vec![])),
                    Documentation::none(),
                ));
            }

            for (i, field) in fields.iter().enumerate() {
                if let Some(mut ty) = Type::load(&field.ty)? {
                    ty.replace_self_with(self_path);
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

        let variant_cfg = Cfg::append(mod_cfg, Cfg::load(&variant.attrs));
        let mut annotations = AnnotationSet::load(&variant.attrs)?;
        if let Some(b) = enum_annotations.bool("derive-ostream") {
            annotations.add_default("derive-ostream", AnnotationValue::Bool(b));
        }
        let body = match variant.fields {
            syn::Fields::Unit => VariantBody::Empty(annotations),
            syn::Fields::Named(ref fields) => {
                let path = Path::new(format!("{}_Body", variant.ident));
                let name = RenameRule::SnakeCase
                    .apply(&variant.ident.to_string(), IdentifierType::StructMember)
                    .into_owned();
                VariantBody::Body {
                    name,
                    body: Struct::new(
                        path,
                        generic_params,
                        parse_fields(is_tagged, &fields.named, self_path)?,
                        is_tagged,
                        true,
                        None,
                        false,
                        false,
                        None,
                        annotations,
                        Documentation::none(),
                    ),
                }
            }
            syn::Fields::Unnamed(ref fields) => {
                let path = Path::new(format!("{}_Body", variant.ident));
                let name = RenameRule::SnakeCase
                    .apply(&variant.ident.to_string(), IdentifierType::StructMember)
                    .into_owned();
                VariantBody::Body {
                    name,
                    body: Struct::new(
                        path,
                        generic_params,
                        parse_fields(is_tagged, &fields.unnamed, self_path)?,
                        is_tagged,
                        true,
                        None,
                        false,
                        true,
                        None,
                        annotations,
                        Documentation::none(),
                    ),
                }
            }
        };

        Ok(EnumVariant::new(
            variant.ident.to_string(),
            discriminant,
            body,
            variant_cfg,
            Documentation::load(&variant.attrs),
        ))
    }

    pub fn new(
        name: String,
        discriminant: Option<i64>,
        body: VariantBody,
        cfg: Option<Cfg>,
        documentation: Documentation,
    ) -> Self {
        let export_name = name.clone();
        Self {
            name,
            export_name,
            discriminant,
            body,
            cfg,
            documentation,
        }
    }

    fn add_dependencies(&self, library: &Library, out: &mut Dependencies) {
        if let VariantBody::Body { ref body, .. } = self.body {
            body.add_dependencies(library, out);
        }
    }

    fn resolve_declaration_types(&mut self, resolver: &DeclarationTypeResolver) {
        if let VariantBody::Body { ref mut body, .. } = self.body {
            body.resolve_declaration_types(resolver);
        }
    }

    fn specialize(
        &self,
        generic_values: &[Type],
        mappings: &[(&Path, &Type)],
        config: &Config,
    ) -> Self {
        Self::new(
            mangle::mangle_name(&self.name, generic_values, &config.export.mangle),
            self.discriminant,
            self.body.specialize(generic_values, mappings, config),
            self.cfg.clone(),
            self.documentation.clone(),
        )
    }

    fn add_monomorphs(&self, library: &Library, out: &mut Monomorphs) {
        if let VariantBody::Body { ref body, .. } = self.body {
            body.add_monomorphs(library, out);
        }
    }

    fn mangle_paths(&mut self, monomorphs: &Monomorphs) {
        if let VariantBody::Body { ref mut body, .. } = self.body {
            body.mangle_paths(monomorphs);
        }
    }
}

impl Source for EnumVariant {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        let condition = self.cfg.to_condition(config);
        condition.write_before(config, out);
        self.documentation.write(config, out);
        write!(out, "{}", self.export_name);
        if let Some(discriminant) = self.discriminant {
            write!(out, " = {}", discriminant);
        }
        out.write(",");
        condition.write_after(config, out);
    }
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub path: Path,
    pub export_name: String,
    pub generic_params: GenericParams,
    pub repr: Repr,
    pub variants: Vec<EnumVariant>,
    pub tag: Option<String>,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}

impl Enum {
    pub fn add_monomorphs(&self, library: &Library, out: &mut Monomorphs) {
        if self.generic_params.len() > 0 {
            return;
        }

        for v in &self.variants {
            v.add_monomorphs(library, out);
        }
    }

    fn can_derive_eq(&self) -> bool {
        if self.tag.is_none() {
            return false;
        }

        self.variants.iter().all(|variant| match variant.body {
            VariantBody::Empty(..) => true,
            VariantBody::Body { ref body, .. } => body.can_derive_eq(),
        })
    }

    pub fn mangle_paths(&mut self, monomorphs: &Monomorphs) {
        for variant in &mut self.variants {
            variant.mangle_paths(monomorphs);
        }
    }

    pub fn load(
        item: &syn::ItemEnum,
        mod_cfg: Option<&Cfg>,
        config: &Config,
    ) -> Result<Enum, String> {
        let repr = Repr::load(&item.attrs)?;
        if repr.style == ReprStyle::Rust && repr.ty.is_none() {
            return Err("Enum is not marked with a valid #[repr(prim)] or #[repr(C)].".to_owned());
        }
        // TODO: Implement translation of aligned enums.
        if repr.align.is_some() {
            return Err("Enum is marked with #[repr(align(...))] or #[repr(packed)].".to_owned());
        }

        let path = Path::new(item.ident.to_string());
        let generic_params = GenericParams::new(&item.generics);

        let mut variants = Vec::new();
        let mut is_tagged = false;

        let annotations = AnnotationSet::load(&item.attrs)?;

        for variant in item.variants.iter() {
            let variant = EnumVariant::load(
                repr.style == ReprStyle::Rust,
                variant,
                generic_params.clone(),
                mod_cfg,
                &path,
                &annotations,
            )?;
            is_tagged = is_tagged || !variant.body.is_empty();
            variants.push(variant);
        }

        if let Some(names) = annotations.list("enum-trailing-values") {
            for name in names {
                variants.push(EnumVariant::new(
                    name,
                    None,
                    VariantBody::empty(),
                    None,
                    Documentation::none(),
                ));
            }
        }

        if config.enumeration.add_sentinel(&annotations) {
            variants.push(EnumVariant::new(
                "Sentinel".to_owned(),
                None,
                VariantBody::empty(),
                None,
                Documentation::simple(" Must be last for serialization purposes"),
            ));
        }

        let tag = if is_tagged {
            Some("Tag".to_string())
        } else {
            None
        };

        Ok(Enum::new(
            path,
            generic_params,
            repr,
            variants,
            tag,
            Cfg::append(mod_cfg, Cfg::load(&item.attrs)),
            annotations,
            Documentation::load(&item.attrs),
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        path: Path,
        generic_params: GenericParams,
        repr: Repr,
        variants: Vec<EnumVariant>,
        tag: Option<String>,
        cfg: Option<Cfg>,
        annotations: AnnotationSet,
        documentation: Documentation,
    ) -> Self {
        let export_name = path.name().to_owned();
        Self {
            path,
            export_name,
            generic_params,
            repr,
            variants,
            tag,
            cfg,
            annotations,
            documentation,
        }
    }
}

impl Item for Enum {
    fn path(&self) -> &Path {
        &self.path
    }

    fn export_name(&self) -> &str {
        &self.export_name
    }

    fn cfg(&self) -> Option<&Cfg> {
        self.cfg.as_ref()
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
            resolver.add_struct(&self.path);
        } else if self.tag.is_some() && self.repr.style != ReprStyle::C {
            resolver.add_union(&self.path);
        } else if self.repr.style == ReprStyle::C {
            resolver.add_enum(&self.path);
        }
    }

    fn resolve_declaration_types(&mut self, resolver: &DeclarationTypeResolver) {
        for &mut ref mut var in &mut self.variants {
            var.resolve_declaration_types(resolver);
        }
    }

    fn rename_for_config(&mut self, config: &Config) {
        config.export.rename(&mut self.export_name);

        if config.language == Language::C && self.tag.is_some() {
            // it makes sense to always prefix Tag with type name in C
            let new_tag = format!("{}_Tag", self.export_name);
            if self.repr.style == ReprStyle::Rust {
                for variant in &mut self.variants {
                    if let VariantBody::Body { ref mut body, .. } = variant.body {
                        let path = Path::new(new_tag.clone());
                        let generic_path = GenericPath::new(path, vec![]);
                        body.fields[0].1 = Type::Path(generic_path);
                    }
                }
            }
            self.tag = Some(new_tag);
        }

        for variant in &mut self.variants {
            reserved::escape(&mut variant.export_name);

            if let VariantBody::Body {
                ref mut name,
                ref mut body,
            } = variant.body
            {
                body.rename_for_config(config);
                reserved::escape(name);
            }
        }

        if config.enumeration.prefix_with_name
            || self.annotations.bool("prefix-with-name").unwrap_or(false)
        {
            for variant in &mut self.variants {
                variant.export_name = format!("{}_{}", self.export_name, variant.export_name);
                if let VariantBody::Body { ref mut body, .. } = variant.body {
                    body.export_name = format!("{}_{}", self.export_name, body.export_name());
                }
            }
        }

        let rules = self
            .annotations
            .parse_atom::<RenameRule>("rename-all")
            .unwrap_or(config.enumeration.rename_variants);

        if let Some(r) = rules.not_none() {
            self.variants = self
                .variants
                .iter()
                .map(|variant| {
                    EnumVariant::new(
                        r.apply(&variant.export_name, IdentifierType::EnumVariant(self))
                            .into_owned(),
                        variant.discriminant,
                        match variant.body {
                            VariantBody::Empty(..) => variant.body.clone(),
                            VariantBody::Body { ref name, ref body } => VariantBody::Body {
                                name: r.apply(&name, IdentifierType::StructMember).into_owned(),
                                body: body.clone(),
                            },
                        },
                        variant.cfg.clone(),
                        variant.documentation.clone(),
                    )
                })
                .collect();
        }
    }

    fn instantiate_monomorph(
        &self,
        generic_values: &[Type],
        library: &Library,
        out: &mut Monomorphs,
    ) {
        assert!(
            self.generic_params.len() > 0,
            "{} is not generic",
            self.path.name()
        );
        assert!(
            self.generic_params.len() == generic_values.len(),
            "{} has {} params but is being instantiated with {} values",
            self.path.name(),
            self.generic_params.len(),
            generic_values.len(),
        );

        let mappings = self
            .generic_params
            .iter()
            .zip(generic_values.iter())
            .collect::<Vec<_>>();

        for variant in &self.variants {
            if let VariantBody::Body { ref body, .. } = variant.body {
                body.instantiate_monomorph(generic_values, library, out);
            }
        }

        let mangled_path = mangle::mangle_path(
            &self.path,
            generic_values,
            &library.get_config().export.mangle,
        );

        let monomorph = Enum::new(
            mangled_path,
            GenericParams::default(),
            self.repr,
            self.variants
                .iter()
                .map(|v| v.specialize(generic_values, &mappings, library.get_config()))
                .collect(),
            self.tag.clone(),
            self.cfg.clone(),
            self.annotations.clone(),
            self.documentation.clone(),
        );

        monomorph.add_monomorphs(library, out);

        out.insert_enum(self, monomorph, generic_values.to_owned());
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
            ReprType::U64 => "uint64_t",
            ReprType::U32 => "uint32_t",
            ReprType::U16 => "uint16_t",
            ReprType::U8 => "uint8_t",
            ReprType::ISize => "intptr_t",
            ReprType::I64 => "int64_t",
            ReprType::I32 => "int32_t",
            ReprType::I16 => "int16_t",
            ReprType::I8 => "int8_t",
        });

        let condition = self.cfg.to_condition(config);
        condition.write_before(config, out);

        self.documentation.write(config, out);

        let is_tagged = self.tag.is_some();
        let separate_tag = self.repr.style == ReprStyle::C;

        // If tagged, we need to emit a proper struct/union wrapper around our enum
        self.generic_params.write(config, out);
        if is_tagged && config.language == Language::Cxx {
            out.write(if separate_tag { "struct" } else { "union" });

            if self.annotations.must_use {
                if let Some(ref anno) = config.structure.must_use {
                    write!(out, " {}", anno)
                }
            }

            write!(out, " {}", self.export_name());
            out.open_brace();

            // Emit the pre_body section, if relevant
            // Only do this here if we're writing C++, since the struct that wraps everything is starting here.
            // If we're writing C, we aren't wrapping the enum and variant structs definitions, so the actual enum struct willstart down below
            if let Some(body) = config.export.pre_body(&self.path) {
                out.write_raw_block(body);
                out.new_line();
            }
        }

        let enum_name = if let Some(ref tag) = self.tag {
            tag
        } else {
            self.export_name()
        };

        // Emit the actual enum
        if config.language == Language::C {
            if size.is_none() && config.style.generate_typedef() {
                out.write("typedef ");
            }

            out.write("enum");

            if size.is_some() || config.style.generate_tag() {
                write!(out, " {}", enum_name);
            }

            if config.cpp_compat {
                if let Some(prim) = size {
                    out.new_line();
                    out.write("#ifdef __cplusplus");
                    out.new_line();
                    write!(out, "  : {}", prim);
                    out.new_line();
                    out.write("#endif // __cplusplus");
                    out.new_line();
                }
            }
        } else {
            if config.enumeration.enum_class(&self.annotations) {
                out.write("enum class");
            } else {
                out.write("enum");
            }

            if self.annotations.must_use {
                if let Some(ref anno) = config.enumeration.must_use {
                    write!(out, " {}", anno)
                }
            }

            write!(out, " {}", enum_name);
            if let Some(prim) = size {
                write!(out, " : {}", prim);
            }
        }
        out.open_brace();
        for (i, variant) in self.variants.iter().enumerate() {
            if i != 0 {
                out.new_line()
            }
            variant.write(config, out);
        }

        if config.language == Language::C && size.is_none() && config.style.generate_typedef() {
            out.close_brace(false);
            write!(out, " {};", enum_name);
        } else {
            out.close_brace(true);
        }

        if config.language == Language::C {
            if let Some(prim) = size {
                if config.cpp_compat {
                    out.new_line_if_not_start();
                    out.write("#ifndef __cplusplus");
                }

                out.new_line();
                write!(out, "typedef {} {};", prim, enum_name);

                if config.cpp_compat {
                    out.new_line_if_not_start();
                    out.write("#endif // __cplusplus");
                }
            }
        }
        // Done emitting the enum

        // Emit an ostream function if required.
        let derive_ostream = config.enumeration.derive_ostream(&self.annotations);
        if config.language == Language::Cxx && derive_ostream {
            // For untagged enums, this emits the serializer function for the
            // enum. For tagged enums, this emits the serializer function for
            // the tag. In the latter case we need a couple of minor changes
            // due to the function living inside the top-level struct or enum.
            let stream = config
                .function
                .rename_args
                .apply("stream", IdentifierType::FunctionArg);
            let instance = config
                .function
                .rename_args
                .apply("instance", IdentifierType::FunctionArg);

            out.new_line();
            out.new_line();
            // For untagged enums, we mark the function inline because the
            // header might get included into multiple compilation units that
            // get linked together, and not marking it inline would result in
            // multiply-defined symbol errors. For tagged enums we don't have
            // the same problem, but mark it as a friend function of the
            // containing union/struct.
            // Note also that for tagged enums, the case labels for switch
            // statements apparently need to be qualified to the top-level
            // generated struct or union. This is why the generated case labels
            // below use the A::B::C format for tagged enums, with A being
            // self.export_name(). Failure to have that qualification results
            // in a surprising compilation failure for the generated header.
            write!(
                out,
                "{} std::ostream& operator<<(std::ostream& {}, const {}& {})",
                if is_tagged { "friend" } else { "inline" },
                stream,
                enum_name,
                instance,
            );

            out.open_brace();
            if is_tagged {
                // C++ name resolution rules are weird.
                write!(
                    out,
                    "using {} = {}::{};",
                    enum_name,
                    self.export_name(),
                    enum_name
                );
                out.new_line();
            }
            write!(out, "switch ({})", instance);
            out.open_brace();
            let vec: Vec<_> = self
                .variants
                .iter()
                .map(|x| {
                    format!(
                        "case {}::{}: {} << \"{}\"; break;",
                        enum_name, x.export_name, stream, x.export_name
                    )
                })
                .collect();
            out.write_vertical_source_list(&vec[..], ListType::Join(""));
            out.close_brace(false);
            out.new_line();

            write!(out, "return {};", stream);
            out.close_brace(false);

            if is_tagged {
                // For tagged enums, this emits the serializer function for
                // the top-level enum or struct.
                out.new_line();
                out.new_line();
                write!(
                    out,
                    "friend std::ostream& operator<<(std::ostream& {}, const {}& {})",
                    stream,
                    self.export_name(),
                    instance,
                );

                out.open_brace();

                // C++ name resolution rules are weird.
                write!(
                    out,
                    "using {} = {}::{};",
                    enum_name,
                    self.export_name(),
                    enum_name
                );
                out.new_line();

                write!(out, "switch ({}.tag)", instance);
                out.open_brace();
                let vec: Vec<_> = self
                    .variants
                    .iter()
                    .map(|x| {
                        let tag_str = format!("\"{}\"", x.export_name);
                        if let VariantBody::Body { ref name, .. } = x.body {
                            format!(
                                "case {}::{}: {} << {}{}{}.{}; break;",
                                enum_name,
                                x.export_name,
                                stream,
                                if separate_tag { &tag_str } else { "" },
                                if separate_tag { " << " } else { "" },
                                instance,
                                name,
                            )
                        } else {
                            format!(
                                "case {}::{}: {} << {}; break;",
                                enum_name, x.export_name, stream, tag_str,
                            )
                        }
                    })
                    .collect();
                out.write_vertical_source_list(&vec[..], ListType::Join(""));
                out.close_brace(false);
                out.new_line();

                write!(out, "return {};", stream);
                out.close_brace(false);
            }
        }

        // If tagged, we need to emit structs for the cases and union them together
        if is_tagged {
            // Emit the cases for the structs
            for variant in &self.variants {
                if let VariantBody::Body { ref body, .. } = variant.body {
                    out.new_line();
                    out.new_line();
                    let condition = variant.cfg.to_condition(config);
                    condition.write_before(config, out);
                    body.write(config, out);
                    condition.write_after(config, out);
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
                    write!(out, " {}", self.export_name());
                }

                out.open_brace();

                // Emit the pre_body section, if relevant
                // Only do this if we're writing C, since the struct is starting right here.
                // For C++, the struct wraps all of the above variant structs too, and we write the pre_body section at the begining of that
                if let Some(body) = config.export.pre_body(&self.path) {
                    out.write_raw_block(body);
                    out.new_line();
                }
            }

            // C++ allows accessing only common initial sequence of union
            // branches so we need to wrap tag into an anonymous struct
            let wrap_tag = config.language == Language::Cxx && !separate_tag;

            if wrap_tag {
                out.write("struct");
                out.open_brace();
            }

            if config.language == Language::C && size.is_none() && !config.style.generate_typedef()
            {
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

            {
                let mut first = true;
                for variant in &self.variants {
                    let (field_name, body) = match variant.body {
                        VariantBody::Body { ref name, ref body } => (name, body),
                        VariantBody::Empty(..) => continue,
                    };

                    if !first {
                        out.new_line();
                    }
                    first = false;
                    let condition = variant.cfg.to_condition(config);
                    condition.write_before(config, out);
                    if config.style.generate_typedef() {
                        write!(out, "{} {};", body.export_name(), field_name);
                    } else {
                        write!(out, "struct {} {};", body.export_name(), field_name);
                    }
                    condition.write_after(config, out);
                }
            }

            if separate_tag {
                out.close_brace(true);
            }

            let skip_fields = if separate_tag { 0 } else { 1 };

            // Emit convenience methods
            let derive_helper_methods = config.enumeration.derive_helper_methods(&self.annotations);
            let derive_const_casts = config.enumeration.derive_const_casts(&self.annotations);
            let derive_mut_casts = config.enumeration.derive_mut_casts(&self.annotations);
            if config.language == Language::Cxx && derive_helper_methods {
                for variant in &self.variants {
                    out.new_line();
                    out.new_line();

                    let condition = variant.cfg.to_condition(config);
                    condition.write_before(config, out);

                    let arg_renamer = |name: &str| {
                        config
                            .function
                            .rename_args
                            .apply(name, IdentifierType::FunctionArg)
                            .into_owned()
                    };

                    macro_rules! write_attrs {
                        ($op:expr) => {{
                            if let Some(Some(attrs)) = variant.body.annotations().atom(concat!(
                                "variant-",
                                $op,
                                "-attributes"
                            )) {
                                write!(out, "{} ", attrs);
                            }
                        }};
                    };

                    write_attrs!("constructor");
                    write!(out, "static {} {}(", self.export_name, variant.export_name);

                    if let VariantBody::Body { ref body, .. } = variant.body {
                        let vec: Vec<_> = body
                            .fields
                            .iter()
                            .skip(skip_fields)
                            .map(|&(ref name, ref ty, _)| {
                                // const-ref args to constructor
                                (arg_renamer(name), Type::const_ref_to(ty))
                            })
                            .collect();
                        out.write_vertical_source_list(&vec[..], ListType::Join(","));
                    }

                    write!(out, ")");
                    out.open_brace();

                    write!(out, "{} result;", self.export_name);

                    if let VariantBody::Body {
                        name: ref variant_name,
                        ref body,
                    } = variant.body
                    {
                        for &(ref field_name, ref ty, ..) in body.fields.iter().skip(skip_fields) {
                            out.new_line();
                            match ty {
                                Type::Array(ref ty, ref length) => {
                                    // arrays are not assignable in C++ so we
                                    // need to manually copy the elements
                                    write!(out, "for (int i = 0; i < {}; i++)", length.as_str());
                                    out.open_brace();
                                    write!(
                                        out,
                                        "::new (&result.{}.{}[i]) (",
                                        variant_name, field_name
                                    );
                                    ty.write(config, out);
                                    write!(out, ")({}[i]);", arg_renamer(field_name));
                                    out.close_brace(false);
                                }
                                ref ty => {
                                    write!(
                                        out,
                                        "::new (&result.{}.{}) (",
                                        variant_name, field_name
                                    );
                                    ty.write(config, out);
                                    write!(out, ")({});", arg_renamer(field_name));
                                }
                            }
                        }
                    }

                    out.new_line();
                    write!(out, "result.tag = {}::{};", enum_name, variant.export_name);
                    out.new_line();
                    write!(out, "return result;");
                    out.close_brace(false);

                    out.new_line();
                    out.new_line();

                    write_attrs!("is");
                    // FIXME: create a config for method case
                    write!(out, "bool Is{}() const", variant.export_name);
                    out.open_brace();
                    write!(out, "return tag == {}::{};", enum_name, variant.export_name);
                    out.close_brace(false);

                    let assert_name = match config.enumeration.cast_assert_name {
                        Some(ref n) => &**n,
                        None => "assert",
                    };

                    let mut derive_casts = |const_casts: bool| {
                        let (member_name, body) = match variant.body {
                            VariantBody::Body { ref name, ref body } => (name, body),
                            VariantBody::Empty(..) => return,
                        };

                        let field_count = body.fields.len() - skip_fields;
                        if field_count == 0 {
                            return;
                        }

                        out.new_line();
                        out.new_line();

                        let dig = field_count == 1 && body.tuple_struct;
                        if const_casts {
                            write_attrs!("const-cast");
                        } else {
                            write_attrs!("mut-cast");
                        }
                        if dig {
                            let field = body.fields.get(skip_fields).unwrap();
                            let return_type = field.1.clone();
                            let return_type = Type::Ptr {
                                ty: Box::new(return_type),
                                is_const: const_casts,
                                is_ref: true,
                                is_nullable: false,
                            };
                            return_type.write(config, out);
                        } else if const_casts {
                            write!(out, "const {}&", body.export_name());
                        } else {
                            write!(out, "{}&", body.export_name());
                        }

                        write!(out, " As{}()", variant.export_name);
                        if const_casts {
                            write!(out, " const");
                        }
                        out.open_brace();
                        write!(out, "{}(Is{}());", assert_name, variant.export_name);
                        out.new_line();
                        if dig {
                            write!(out, "return {}._0;", member_name);
                        } else {
                            write!(out, "return {};", member_name);
                        }
                        out.close_brace(false);
                    };

                    if derive_const_casts {
                        derive_casts(true)
                    }

                    if derive_mut_casts {
                        derive_casts(false)
                    }

                    condition.write_after(config, out);
                }
            }

            let other = config
                .function
                .rename_args
                .apply("other", IdentifierType::FunctionArg);

            macro_rules! write_attrs {
                ($op:expr) => {{
                    if let Some(Some(attrs)) = self.annotations.atom(concat!($op, "-attributes")) {
                        write!(out, "{} ", attrs);
                    }
                }};
            };

            if config.language == Language::Cxx
                && self.can_derive_eq()
                && config.structure.derive_eq(&self.annotations)
            {
                out.new_line();
                out.new_line();
                write_attrs!("eq");
                write!(
                    out,
                    "bool operator==(const {}& {}) const",
                    self.export_name, other
                );
                out.open_brace();
                write!(out, "if (tag != {}.tag)", other);
                out.open_brace();
                write!(out, "return false;");
                out.close_brace(false);
                out.new_line();
                write!(out, "switch (tag)");
                out.open_brace();
                let mut exhaustive = true;
                for variant in &self.variants {
                    if let VariantBody::Body {
                        name: ref variant_name,
                        ..
                    } = variant.body
                    {
                        let condition = variant.cfg.to_condition(config);
                        condition.write_before(config, out);
                        write!(
                            out,
                            "case {}::{}: return {} == {}.{};",
                            self.tag.as_ref().unwrap(),
                            variant.export_name,
                            variant_name,
                            other,
                            variant_name
                        );
                        condition.write_after(config, out);
                        out.new_line();
                    } else {
                        exhaustive = false;
                    }
                }
                if !exhaustive {
                    write!(out, "default: break;");
                }
                out.close_brace(false);

                out.new_line();
                write!(out, "return true;");

                out.close_brace(false);

                if config.structure.derive_neq(&self.annotations) {
                    out.new_line();
                    out.new_line();
                    write_attrs!("neq");
                    write!(
                        out,
                        "bool operator!=(const {}& {}) const",
                        self.export_name, other
                    );
                    out.open_brace();
                    write!(out, "return !(*this == {});", other);
                    out.close_brace(false);
                }
            }

            if config.language == Language::Cxx
                && config
                    .enumeration
                    .private_default_tagged_enum_constructor(&self.annotations)
            {
                out.new_line();
                out.new_line();
                write!(out, "private:");
                out.new_line();
                write!(out, "{}()", self.export_name);
                out.open_brace();
                out.close_brace(false);
                out.new_line();
                write!(out, "public:");
                out.new_line();
            }

            if config.language == Language::Cxx
                && config
                    .enumeration
                    .derive_tagged_enum_destructor(&self.annotations)
            {
                out.new_line();
                out.new_line();
                write_attrs!("destructor");
                write!(out, "~{}()", self.export_name);
                out.open_brace();
                write!(out, "switch (tag)");
                out.open_brace();
                let mut exhaustive = true;
                for variant in &self.variants {
                    if let VariantBody::Body { ref name, ref body } = variant.body {
                        let condition = variant.cfg.to_condition(config);
                        condition.write_before(config, out);
                        write!(
                            out,
                            "case {}::{}: {}.~{}(); break;",
                            self.tag.as_ref().unwrap(),
                            variant.export_name,
                            name,
                            body.export_name(),
                        );
                        condition.write_after(config, out);
                        out.new_line();
                    } else {
                        exhaustive = false;
                    }
                }
                if !exhaustive {
                    write!(out, "default: break;");
                }
                out.close_brace(false);
                out.close_brace(false);
            }

            if config.language == Language::Cxx
                && config
                    .enumeration
                    .derive_tagged_enum_copy_constructor(&self.annotations)
            {
                out.new_line();
                out.new_line();
                write_attrs!("copy-constructor");
                write!(
                    out,
                    "{}(const {}& {})",
                    self.export_name, self.export_name, other
                );
                out.new_line();
                write!(out, " : tag({}.tag)", other);
                out.open_brace();
                write!(out, "switch (tag)");
                out.open_brace();
                let mut exhaustive = true;
                for variant in &self.variants {
                    if let VariantBody::Body { ref name, ref body } = variant.body {
                        let condition = variant.cfg.to_condition(config);
                        condition.write_before(config, out);
                        write!(
                            out,
                            "case {}::{}: ::new (&{}) ({})({}.{}); break;",
                            self.tag.as_ref().unwrap(),
                            variant.export_name,
                            name,
                            body.export_name(),
                            other,
                            name,
                        );
                        condition.write_after(config, out);
                        out.new_line();
                    } else {
                        exhaustive = false;
                    }
                }
                if !exhaustive {
                    write!(out, "default: break;");
                }
                out.close_brace(false);
                out.close_brace(false);

                if config.language == Language::Cxx
                    && config
                        .enumeration
                        .derive_tagged_enum_copy_assignment(&self.annotations)
                {
                    out.new_line();
                    write_attrs!("copy-assignment");
                    write!(
                        out,
                        "{}& operator=(const {}& {})",
                        self.export_name, self.export_name, other
                    );
                    out.open_brace();
                    write!(out, "if (this != &{})", other);
                    out.open_brace();
                    write!(out, "this->~{}();", self.export_name);
                    out.new_line();
                    write!(out, "new (this) {}({});", self.export_name, other);
                    out.close_brace(false);
                    out.new_line();
                    write!(out, "return *this;");
                    out.close_brace(false);
                }
            }

            // Emit the post_body section, if relevant
            if let Some(body) = config.export.post_body(&self.path) {
                out.new_line();
                out.write_raw_block(body);
            }

            if config.language == Language::C && config.style.generate_typedef() {
                out.close_brace(false);
                write!(out, " {};", self.export_name);
            } else {
                out.close_brace(true);
            }
        }
        condition.write_after(config, out);
    }
}
