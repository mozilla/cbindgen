/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use syn;

use bindgen::config::{Config, Language, LayoutConfig};
use bindgen::declarationtyperesolver::DeclarationTypeResolver;
use bindgen::dependencies::Dependencies;
use bindgen::ir::{
    AnnotationSet, Cfg, ConditionWrite, Constant, Documentation, GenericParams, Item,
    ItemContainer, Path, Repr, ReprAlign, ReprStyle, ToCondition, Type, Typedef,
};
use bindgen::library::Library;
use bindgen::mangle;
use bindgen::monomorph::Monomorphs;
use bindgen::rename::{IdentifierType, RenameRule};
use bindgen::reserved;
use bindgen::utilities::{find_first_some, IterHelpers};
use bindgen::writer::{ListType, Source, SourceWriter};

#[derive(Debug, Clone)]
pub struct Struct {
    pub path: Path,
    pub export_name: String,
    pub generic_params: GenericParams,
    pub fields: Vec<(String, Type, Documentation)>,
    /// Whether there's a tag field on the body of this struct. When this is
    /// true, is_enum_variant_body is also guaranteed to be true.
    pub is_tagged: bool,
    /// Whether this is an enum variant body.
    pub is_enum_variant_body: bool,
    pub alignment: Option<ReprAlign>,
    pub is_transparent: bool,
    pub tuple_struct: bool,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
    pub associated_constants: Vec<Constant>,
}

impl Struct {
    /// Whether this struct can derive operator== / operator!=.
    pub fn can_derive_eq(&self) -> bool {
        !self.fields.is_empty() && self.fields.iter().all(|x| x.1.can_cmp_eq())
    }

    pub fn add_associated_constant(&mut self, c: Constant) {
        self.associated_constants.push(c);
    }

    pub fn load(
        layout_config: &LayoutConfig,
        item: &syn::ItemStruct,
        mod_cfg: Option<&Cfg>,
    ) -> Result<Self, String> {
        let repr = Repr::load(&item.attrs)?;
        let is_transparent = match repr.style {
            ReprStyle::C => false,
            ReprStyle::Transparent => true,
            _ => {
                return Err("Struct is not marked #[repr(C)] or #[repr(transparent)].".to_owned());
            }
        };

        // Ensure we can safely represent the struct given the configuration.
        if let Some(align) = repr.align {
            layout_config.ensure_safe_to_represent(&align)?;
        }

        let (fields, tuple_struct) = match item.fields {
            syn::Fields::Unit => (Vec::new(), false),
            syn::Fields::Named(ref fields) => {
                let out = fields
                    .named
                    .iter()
                    .try_skip_map(|x| x.as_ident_and_type())?;
                (out, false)
            }
            syn::Fields::Unnamed(ref fields) => {
                let mut out = Vec::new();
                let mut current = 0;
                for field in fields.unnamed.iter() {
                    if let Some(x) = Type::load(&field.ty)? {
                        out.push((format!("{}", current), x, Documentation::load(&field.attrs)));
                        current += 1;
                    }
                }
                (out, true)
            }
        };

        let is_tagged = false;
        let is_enum_variant_body = false;

        Ok(Struct::new(
            Path::new(item.ident.to_string()),
            GenericParams::new(&item.generics),
            fields,
            is_tagged,
            is_enum_variant_body,
            repr.align,
            is_transparent,
            tuple_struct,
            Cfg::append(mod_cfg, Cfg::load(&item.attrs)),
            AnnotationSet::load(&item.attrs)?,
            Documentation::load(&item.attrs),
        ))
    }

    pub fn new(
        path: Path,
        generic_params: GenericParams,
        fields: Vec<(String, Type, Documentation)>,
        is_tagged: bool,
        is_enum_variant_body: bool,
        alignment: Option<ReprAlign>,
        is_transparent: bool,
        tuple_struct: bool,
        cfg: Option<Cfg>,
        annotations: AnnotationSet,
        documentation: Documentation,
    ) -> Self {
        let export_name = path.name().to_owned();
        Self {
            path,
            export_name,
            generic_params,
            fields,
            is_tagged,
            is_enum_variant_body,
            alignment,
            is_transparent,
            tuple_struct,
            cfg,
            annotations,
            documentation,
            associated_constants: vec![],
        }
    }

    pub fn simplify_standard_types(&mut self) {
        for &mut (_, ref mut ty, _) in &mut self.fields {
            ty.simplify_standard_types();
        }
    }

    pub fn is_generic(&self) -> bool {
        self.generic_params.len() > 0
    }

    pub fn add_monomorphs(&self, library: &Library, out: &mut Monomorphs) {
        // Generic structs can instantiate monomorphs only once they've been
        // instantiated. See `instantiate_monomorph` for more details.
        if self.is_generic() {
            return;
        }

        for &(_, ref ty, _) in &self.fields {
            ty.add_monomorphs(library, out);
        }
    }

    pub fn mangle_paths(&mut self, monomorphs: &Monomorphs) {
        for &mut (_, ref mut ty, _) in &mut self.fields {
            ty.mangle_paths(monomorphs);
        }
    }

    pub fn specialize(&self, generic_values: &[Type], mappings: &[(&Path, &Type)]) -> Self {
        let mangled_path = mangle::mangle_path(&self.path, generic_values);
        Struct::new(
            mangled_path,
            GenericParams::default(),
            self.fields
                .iter()
                .map(|x| (x.0.clone(), x.1.specialize(mappings), x.2.clone()))
                .collect(),
            self.is_tagged,
            self.is_enum_variant_body,
            self.alignment,
            self.is_transparent,
            self.tuple_struct,
            self.cfg.clone(),
            self.annotations.clone(),
            self.documentation.clone(),
        )
    }

    fn emit_bitflags_binop<F: Write>(
        &self,
        operator: char,
        other: &str,
        out: &mut SourceWriter<F>,
    ) {
        out.new_line();
        write!(
            out,
            "{} operator{}(const {}& {}) const",
            self.export_name(),
            operator,
            self.export_name(),
            other
        );
        out.open_brace();
        write!(
            out,
            "return {{static_cast<decltype(bits)>(this->bits {} {}.bits)}};",
            operator, other
        );
        out.close_brace(false);

        out.new_line();
        write!(
            out,
            "{}& operator{}=(const {}& {})",
            self.export_name(),
            operator,
            self.export_name(),
            other
        );
        out.open_brace();
        write!(out, "*this = (*this {} {});", operator, other);
        out.new_line();
        write!(out, "return *this;");
        out.close_brace(false);
    }
}

impl Item for Struct {
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
        ItemContainer::Struct(self.clone())
    }

    fn collect_declaration_types(&self, resolver: &mut DeclarationTypeResolver) {
        if !self.is_transparent {
            resolver.add_struct(&self.path);
        }
    }

    fn resolve_declaration_types(&mut self, resolver: &DeclarationTypeResolver) {
        for &mut (_, ref mut ty, _) in &mut self.fields {
            ty.resolve_declaration_types(resolver);
        }
    }

    fn rename_for_config(&mut self, config: &Config) {
        // Rename the name of the struct
        if !self.is_tagged || config.language == Language::C {
            config.export.rename(&mut self.export_name);
        }

        // Rename the types used in fields
        {
            let fields = self
                .fields
                .iter_mut()
                .skip(if self.is_tagged { 1 } else { 0 });
            for &mut (_, ref mut ty, _) in fields {
                ty.rename_for_config(config, &self.generic_params);
            }
        }

        // Apply renaming rules to fields in the following order
        //   1. `cbindgen::field-names` annotation
        //   2. `cbindgen::rename-all` annotation
        //   3. config struct rename rule
        // If the struct is a tuple struct and we have not renamed the
        // fields, then prefix each of them with an underscore.
        // If any field is a reserved keyword, then postfix it with an
        // underscore.

        // Scope for mutable borrow of fields
        {
            let mut names = self.fields.iter_mut().map(|field| &mut field.0);

            let field_rules = [
                self.annotations.parse_atom::<RenameRule>("rename-all"),
                config.structure.rename_fields,
            ];

            if let Some(o) = self.annotations.list("field-names") {
                for (dest, src) in names.zip(o) {
                    *dest = src;
                }
            } else if let Some(r) = find_first_some(&field_rules) {
                for name in names {
                    *name = r.apply_to_snake_case(name, IdentifierType::StructMember);
                }
            } else if self.tuple_struct {
                // If there is a tag field, skip it
                if self.is_tagged {
                    names.next();
                }

                // If we don't have any rules for a tuple struct, prefix them with
                // an underscore so it still compiles
                for name in names {
                    name.insert(0, '_');
                }
            }
        }

        for field in &mut self.fields {
            reserved::escape(&mut field.0);
        }

        for c in self.associated_constants.iter_mut() {
            c.rename_for_config(config);
        }
    }

    fn add_dependencies(&self, library: &Library, out: &mut Dependencies) {
        let mut fields = self.fields.iter();

        // If there is a tag field, skip it
        if self.is_tagged {
            fields.next();
        }

        for &(_, ref ty, _) in fields {
            ty.add_dependencies_ignoring_generics(&self.generic_params, library, out);
        }

        for c in &self.associated_constants {
            c.add_dependencies(library, out);
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
            self.path
        );
        assert!(
            self.generic_params.len() == generic_values.len(),
            "{} has {} params but is being instantiated with {} values",
            self.path,
            self.generic_params.len(),
            generic_values.len(),
        );

        let mappings = self
            .generic_params
            .iter()
            .zip(generic_values.iter())
            .collect::<Vec<_>>();

        let monomorph = self.specialize(generic_values, &mappings);

        // Instantiate any monomorphs for any generic paths we may have just created.
        monomorph.add_monomorphs(library, out);

        out.insert_struct(self, monomorph, generic_values.to_owned());
    }
}

impl Source for Struct {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        if self.is_transparent {
            let typedef = Typedef {
                path: self.path.clone(),
                export_name: self.export_name.to_owned(),
                generic_params: self.generic_params.clone(),
                aliased: self.fields[0].1.clone(),
                cfg: self.cfg.clone(),
                annotations: self.annotations.clone(),
                documentation: self.documentation.clone(),
            };
            typedef.write(config, out);
            for constant in &self.associated_constants {
                out.new_line();
                constant.write(config, out, Some(self));
            }
            return;
        }

        let condition = (&self.cfg).to_condition(config);
        condition.write_before(config, out);

        self.documentation.write(config, out);

        if !self.is_enum_variant_body {
            self.generic_params.write(config, out);
        }

        // The following results in
        // C++ or C with Tag as style:
        //   struct Name {
        // C with Type only style:
        //   typedef struct {
        // C with Both as style:
        //   typedef struct Name {
        if config.language == Language::C && config.style.generate_typedef() {
            out.write("typedef ");
        }

        out.write("struct");

        if let Some(align) = self.alignment {
            match align {
                ReprAlign::Packed => {
                    if let Some(ref anno) = config.layout.packed {
                        write!(out, " {}", anno);
                    }
                }
                ReprAlign::Align(n) => {
                    if let Some(ref anno) = config.layout.aligned_n {
                        write!(out, " {}({})", anno, n);
                    }
                }
            }
        }

        if self.annotations.must_use {
            if let Some(ref anno) = config.structure.must_use {
                write!(out, " {}", anno);
            }
        }

        if config.language == Language::Cxx || config.style.generate_tag() {
            write!(out, " {}", self.export_name());
        }

        out.open_brace();

        if config.documentation {
            out.write_vertical_source_list(&self.fields, ListType::Cap(";"));
        } else {
            let vec: Vec<_> = self
                .fields
                .iter()
                .map(|&(ref name, ref ty, _)| (name.clone(), ty.clone()))
                .collect();
            out.write_vertical_source_list(&vec[..], ListType::Cap(";"));
        }

        if config.language == Language::Cxx {
            let mut wrote_start_newline = false;

            if config.structure.derive_constructor(&self.annotations) && !self.fields.is_empty() {
                if !wrote_start_newline {
                    wrote_start_newline = true;
                    out.new_line();
                }

                out.new_line();

                let arg_renamer = |name: &str| {
                    config
                        .function
                        .rename_args
                        .as_ref()
                        .unwrap_or(&RenameRule::GeckoCase)
                        .apply_to_snake_case(name, IdentifierType::FunctionArg)
                };
                write!(out, "{}(", self.export_name());
                let vec: Vec<_> = self
                    .fields
                    .iter()
                    .map(|&(ref name, ref ty, _)| {
                        // const-ref args to constructor
                        (format!("const& {}", arg_renamer(name)), ty.clone())
                    })
                    .collect();
                out.write_vertical_source_list(&vec[..], ListType::Join(","));
                write!(out, ")");
                out.new_line();
                write!(out, "  : ");
                let vec: Vec<_> = self
                    .fields
                    .iter()
                    .map(|x| format!("{}({})", x.0, arg_renamer(&x.0)))
                    .collect();
                out.write_vertical_source_list(&vec[..], ListType::Join(","));
                out.new_line();
                write!(out, "{{}}");
                out.new_line();
            }

            let other = if let Some(r) = config.function.rename_args {
                r.apply_to_snake_case("other", IdentifierType::FunctionArg)
            } else {
                String::from("other")
            };

            if self
                .annotations
                .bool("internal-derive-bitflags")
                .unwrap_or(false)
            {
                if !wrote_start_newline {
                    wrote_start_newline = true;
                    out.new_line();
                }
                out.new_line();
                write!(out, "explicit operator bool() const");
                out.open_brace();
                write!(out, "return !!bits;");
                out.close_brace(false);

                out.new_line();
                write!(out, "{} operator~() const", self.export_name());
                out.open_brace();
                write!(out, "return {{static_cast<decltype(bits)>(~bits)}};");
                out.close_brace(false);

                self.emit_bitflags_binop('|', &other, out);
                self.emit_bitflags_binop('&', &other, out);
                self.emit_bitflags_binop('^', &other, out);
            }

            let skip_fields = if self.is_tagged { 1 } else { 0 };

            let mut emit_op = |op, conjuc| {
                if !wrote_start_newline {
                    wrote_start_newline = true;
                    out.new_line();
                }

                out.new_line();

                write!(
                    out,
                    "bool operator{}(const {}& {}) const",
                    op,
                    self.export_name(),
                    other
                );
                out.open_brace();
                out.write("return ");
                let vec: Vec<_> = self
                    .fields
                    .iter()
                    .skip(skip_fields)
                    .map(|x| format!("{} {} {}.{}", x.0, op, other, x.0))
                    .collect();
                out.write_vertical_source_list(&vec[..], ListType::Join(&format!(" {}", conjuc)));
                out.write(";");
                out.close_brace(false);
            };

            if config.structure.derive_eq(&self.annotations) && self.can_derive_eq() {
                emit_op("==", "&&");
            }
            if config.structure.derive_neq(&self.annotations) && self.can_derive_eq() {
                emit_op("!=", "||");
            }
            if config.structure.derive_lt(&self.annotations)
                && self.fields.len() == 1
                && self.fields[0].1.can_cmp_order()
            {
                emit_op("<", "&&");
            }
            if config.structure.derive_lte(&self.annotations)
                && self.fields.len() == 1
                && self.fields[0].1.can_cmp_order()
            {
                emit_op("<=", "&&");
            }
            if config.structure.derive_gt(&self.annotations)
                && self.fields.len() == 1
                && self.fields[0].1.can_cmp_order()
            {
                emit_op(">", "&&");
            }
            if config.structure.derive_gte(&self.annotations)
                && self.fields.len() == 1
                && self.fields[0].1.can_cmp_order()
            {
                emit_op(">=", "&&");
            }
        }

        if let Some(body) = config.export.extra_body(&self.path) {
            out.write_raw_block(body);
        }

        if config.language == Language::Cxx
            && config.structure.associated_constants_in_body
            && config.constant.allow_static_const
        {
            for constant in &self.associated_constants {
                out.new_line();
                constant.write_declaration(config, out, self);
            }
        }

        if config.language == Language::C && config.style.generate_typedef() {
            out.close_brace(false);
            write!(out, " {};", self.export_name());
        } else {
            out.close_brace(true);
        }

        for constant in &self.associated_constants {
            out.new_line();
            constant.write(config, out, Some(self));
        }

        condition.write_after(config, out);
    }
}

pub trait SynFieldHelpers {
    fn as_ident_and_type(&self) -> Result<Option<(String, Type, Documentation)>, String>;
}

impl SynFieldHelpers for syn::Field {
    fn as_ident_and_type(&self) -> Result<Option<(String, Type, Documentation)>, String> {
        let ident = self
            .ident
            .as_ref()
            .ok_or_else(|| "field is missing identifier".to_string())?
            .clone();
        let converted_ty = Type::load(&self.ty)?;

        if let Some(x) = converted_ty {
            Ok(Some((
                ident.to_string(),
                x,
                Documentation::load(&self.attrs),
            )))
        } else {
            Ok(None)
        }
    }
}
