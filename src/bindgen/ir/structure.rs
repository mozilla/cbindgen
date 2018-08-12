/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use syn;

use bindgen::config::{Config, Language};
use bindgen::declarationtyperesolver::DeclarationTypeResolver;
use bindgen::dependencies::Dependencies;
use bindgen::ir::{
    AnnotationSet, Cfg, ConditionWrite, Documentation, GenericParams, Item, ItemContainer, Repr,
    ToCondition, Type,
};
use bindgen::library::Library;
use bindgen::mangle;
use bindgen::monomorph::Monomorphs;
use bindgen::rename::{IdentifierType, RenameRule};
use bindgen::utilities::{find_first_some, IterHelpers};
use bindgen::writer::{ListType, Source, SourceWriter};

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub generic_params: GenericParams,
    pub fields: Vec<(String, Type, Documentation)>,
    pub is_tagged: bool,
    pub tuple_struct: bool,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}

impl Struct {
    pub fn load(item: &syn::ItemStruct, mod_cfg: &Option<Cfg>) -> Result<Struct, String> {
        if Repr::load(&item.attrs)? != Repr::C {
            return Err("Struct is not marked #[repr(C)].".to_owned());
        }

        let (fields, tuple_struct) = match &item.fields {
            &syn::Fields::Unit => (Vec::new(), false),
            &syn::Fields::Named(ref fields) => {
                let out = fields
                    .named
                    .iter()
                    .try_skip_map(|x| x.as_ident_and_type())?;
                (out, false)
            }
            &syn::Fields::Unnamed(ref fields) => {
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

        Ok(Struct {
            name: item.ident.to_string(),
            generic_params: GenericParams::new(&item.generics),
            fields: fields,
            is_tagged: false,
            tuple_struct: tuple_struct,
            cfg: Cfg::append(mod_cfg, Cfg::load(&item.attrs)),
            annotations: AnnotationSet::load(&item.attrs)?,
            documentation: Documentation::load(&item.attrs),
        })
    }

    pub fn simplify_option_to_ptr(&mut self) {
        for &mut (_, ref mut ty, _) in &mut self.fields {
            ty.simplify_option_to_ptr();
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
}

impl Item for Struct {
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
        ItemContainer::Struct(self.clone())
    }

    fn collect_declaration_types(&self, resolver: &mut DeclarationTypeResolver) {
        resolver.add_struct(&self.name);
    }

    fn resolve_declaration_types(&mut self, resolver: &DeclarationTypeResolver) {
        for &mut (_, ref mut ty, _) in &mut self.fields {
            ty.resolve_declaration_types(resolver);
        }
    }

    fn rename_for_config(&mut self, config: &Config) {
        config.export.rename(&mut self.name);
        for &mut (_, ref mut ty, _) in &mut self.fields {
            let generic_parameter = match ty.get_root_path() {
                Some(ref p) => self.generic_params.contains(p),
                None => false,
            };
            if !generic_parameter {
                ty.rename_for_config(config);
            }
        }

        let field_rules = [
            self.annotations.parse_atom::<RenameRule>("rename-all"),
            config.structure.rename_fields,
        ];

        let mut names = self.fields.iter_mut().map(|field| &mut field.0);

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

    fn add_dependencies(&self, library: &Library, out: &mut Dependencies) {
        let mut fields = self.fields.iter();

        // If there is a tag field, skip it
        if self.is_tagged {
            fields.next();
        }

        for &(_, ref ty, _) in fields {
            ty.add_dependencies_ignoring_generics(&self.generic_params, library, out);
        }
    }

    fn instantiate_monomorph(
        &self,
        generic_values: &Vec<Type>,
        library: &Library,
        out: &mut Monomorphs,
    ) {
        assert!(
            self.generic_params.len() > 0,
            "{} is not generic",
            self.name
        );
        assert!(
            self.generic_params.len() == generic_values.len(),
            "{} has {} params but is being instantiated with {} values",
            self.name,
            self.generic_params.len(),
            generic_values.len(),
        );

        let mappings = self
            .generic_params
            .iter()
            .zip(generic_values.iter())
            .collect::<Vec<_>>();

        let monomorph = Struct {
            name: mangle::mangle_path(&self.name, generic_values),
            generic_params: GenericParams::default(),
            fields: self
                .fields
                .iter()
                .map(|x| (x.0.clone(), x.1.specialize(&mappings), x.2.clone()))
                .collect(),
            is_tagged: self.is_tagged,
            tuple_struct: self.tuple_struct,
            cfg: self.cfg.clone(),
            annotations: self.annotations.clone(),
            documentation: self.documentation.clone(),
        };

        // Instantiate any monomorphs for any generic paths we may have just created.
        monomorph.add_monomorphs(library, out);

        out.insert_struct(self, monomorph, generic_values.clone());
    }
}

impl Source for Struct {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        let condition = (&self.cfg).to_condition(config);
        condition.write_before(config, out);

        self.documentation.write(config, out);

        self.generic_params.write(config, out);

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

        if config.language == Language::Cxx || config.style.generate_tag() {
            write!(out, " {}", self.name);
        }

        out.open_brace();

        if config.documentation {
            out.write_vertical_source_list(&self.fields, ListType::Cap(";"));
        } else {
            out.write_vertical_source_list(
                &self
                    .fields
                    .iter()
                    .map(|&(ref name, ref ty, _)| (name.clone(), ty.clone()))
                    .collect(),
                ListType::Cap(";"),
            );
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
                write!(out, "{}(", self.name);
                out.write_vertical_source_list(
                    &self
                        .fields
                        .iter()
                        .map(|&(ref name, ref ty, _)| {
                            // const-ref args to constructor
                            (format!("const& {}", arg_renamer(name)), ty.clone())
                        }).collect(),
                    ListType::Join(","),
                );
                write!(out, ")");
                out.new_line();
                write!(out, "  : ");
                out.write_vertical_source_list(
                    &self
                        .fields
                        .iter()
                        .map(|x| format!("{}({})", x.0, arg_renamer(&x.0)))
                        .collect(),
                    ListType::Join(","),
                );
                out.new_line();
                write!(out, "{{}}");
                out.new_line();
            }

            let other = if let Some(r) = config.function.rename_args {
                r.apply_to_snake_case("other", IdentifierType::FunctionArg)
            } else {
                String::from("other")
            };

            let mut emit_op = |op, conjuc| {
                if !wrote_start_newline {
                    wrote_start_newline = true;
                    out.new_line();
                }

                out.new_line();

                write!(
                    out,
                    "bool operator{}(const {}& {}) const",
                    op, self.name, other
                );
                out.open_brace();
                out.write("return ");
                out.write_vertical_source_list(
                    &self
                        .fields
                        .iter()
                        .map(|x| format!("{} {} {}.{}", x.0, op, other, x.0))
                        .collect(),
                    ListType::Join(&format!(" {}", conjuc)),
                );
                out.write(";");
                out.close_brace(false);
            };

            if config.structure.derive_eq(&self.annotations)
                && !self.fields.is_empty()
                && self.fields.iter().all(|x| x.1.can_cmp_eq())
            {
                emit_op("==", "&&");
            }
            if config.structure.derive_neq(&self.annotations)
                && !self.fields.is_empty()
                && self.fields.iter().all(|x| x.1.can_cmp_eq())
            {
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

        if config.language == Language::C && config.style.generate_typedef() {
            out.close_brace(false);
            write!(out, " {};", self.name);
        } else {
            out.close_brace(true);
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
            .ok_or(format!("field is missing identifier"))?
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
