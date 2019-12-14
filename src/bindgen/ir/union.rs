/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use syn;

use bindgen::config::{Config, Language, LayoutConfig};
use bindgen::declarationtyperesolver::DeclarationTypeResolver;
use bindgen::dependencies::Dependencies;
use bindgen::ir::SynFieldHelpers;
use bindgen::ir::{
    AnnotationSet, Cfg, ConditionWrite, Documentation, GenericParams, Item, ItemContainer, Path,
    Repr, ReprAlign, ReprStyle, ToCondition, Type,
};
use bindgen::library::Library;
use bindgen::mangle;
use bindgen::monomorph::Monomorphs;
use bindgen::rename::{IdentifierType, RenameRule};
use bindgen::utilities::{find_first_some, IterHelpers};
use bindgen::writer::{ListType, Source, SourceWriter};

#[derive(Debug, Clone)]
pub struct Union {
    pub path: Path,
    pub export_name: String,
    pub generic_params: GenericParams,
    pub fields: Vec<(String, Type, Documentation)>,
    pub tuple_union: bool,
    pub alignment: Option<ReprAlign>,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}

impl Union {
    pub fn load(
        layout_config: &LayoutConfig,
        item: &syn::ItemUnion,
        mod_cfg: Option<&Cfg>,
    ) -> Result<Union, String> {
        let repr = Repr::load(&item.attrs)?;
        if repr.style != ReprStyle::C {
            return Err("Union is not marked #[repr(C)].".to_owned());
        }

        // Ensure we can safely represent the union given the configuration.
        if let Some(align) = repr.align {
            layout_config.ensure_safe_to_represent(&align)?;
        }

        let (fields, tuple_union) = {
            let out = item
                .fields
                .named
                .iter()
                .try_skip_map(|x| x.as_ident_and_type())?;
            (out, false)
        };

        Ok(Union::new(
            Path::new(item.ident.to_string()),
            GenericParams::new(&item.generics),
            fields,
            repr.align,
            tuple_union,
            Cfg::append(mod_cfg, Cfg::load(&item.attrs)),
            AnnotationSet::load(&item.attrs)?,
            Documentation::load(&item.attrs),
        ))
    }

    pub fn new(
        path: Path,
        generic_params: GenericParams,
        fields: Vec<(String, Type, Documentation)>,
        alignment: Option<ReprAlign>,
        tuple_union: bool,
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
            alignment,
            tuple_union,
            cfg,
            annotations,
            documentation,
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
        // Generic unions can instantiate monomorphs only once they've been
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

impl Item for Union {
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
        ItemContainer::Union(self.clone())
    }

    fn collect_declaration_types(&self, resolver: &mut DeclarationTypeResolver) {
        resolver.add_union(&self.path);
    }

    fn resolve_declaration_types(&mut self, resolver: &DeclarationTypeResolver) {
        for &mut (_, ref mut ty, _) in &mut self.fields {
            ty.resolve_declaration_types(resolver);
        }
    }

    fn rename_for_config(&mut self, config: &Config) {
        config.export.rename(&mut self.export_name);
        for &mut (_, ref mut ty, _) in &mut self.fields {
            ty.rename_for_config(config, &self.generic_params);
        }

        let rules = [
            self.annotations.parse_atom::<RenameRule>("rename-all"),
            config.structure.rename_fields,
        ];

        if let Some(o) = self.annotations.list("field-names") {
            let mut overriden_fields = Vec::new();

            for (i, &(ref name, ref ty, ref doc)) in self.fields.iter().enumerate() {
                if i >= o.len() {
                    overriden_fields.push((name.clone(), ty.clone(), doc.clone()));
                } else {
                    overriden_fields.push((o[i].clone(), ty.clone(), doc.clone()));
                }
            }

            self.fields = overriden_fields;
        } else if let Some(r) = find_first_some(&rules) {
            self.fields = self
                .fields
                .iter()
                .map(|x| {
                    (
                        r.apply_to_snake_case(&x.0, IdentifierType::StructMember),
                        x.1.clone(),
                        x.2.clone(),
                    )
                })
                .collect();
        } else if self.tuple_union {
            // If we don't have any rules for a tuple union, prefix them with
            // an underscore so it still compiles
            for &mut (ref mut name, ..) in &mut self.fields {
                name.insert(0, '_');
            }
        }
    }

    fn add_dependencies(&self, library: &Library, out: &mut Dependencies) {
        for &(_, ref ty, _) in &self.fields {
            ty.add_dependencies_ignoring_generics(&self.generic_params, library, out);
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

        let mangled_path = mangle::mangle_path(&self.path, generic_values);
        let monomorph = Union::new(
            mangled_path,
            GenericParams::default(),
            self.fields
                .iter()
                .map(|x| (x.0.clone(), x.1.specialize(&mappings), x.2.clone()))
                .collect(),
            self.alignment,
            self.tuple_union,
            self.cfg.clone(),
            self.annotations.clone(),
            self.documentation.clone(),
        );

        // Instantiate any monomorphs for any generic paths we may have just created.
        monomorph.add_monomorphs(library, out);

        out.insert_union(self, monomorph, generic_values.to_owned());
    }
}

impl Source for Union {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        let condition = (&self.cfg).to_condition(config);
        condition.write_before(config, out);

        self.documentation.write(config, out);

        self.generic_params.write(config, out);

        // The following results in
        // C++ or C with Tag as style:
        //   union Name {
        // C with Type only style:
        //   typedef union {
        // C with Both as style:
        //   typedef union Name {
        if config.language == Language::C && config.style.generate_typedef() {
            out.write("typedef ");
        }

        out.write("union");

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

        if config.language == Language::Cxx || config.style.generate_tag() {
            write!(out, " {}", self.export_name);
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

        if let Some(body) = config.export.extra_body(&self.path) {
            out.write_raw_block(body);
        }

        if config.language == Language::C && config.style.generate_typedef() {
            out.close_brace(false);
            write!(out, " {};", self.export_name);
        } else {
            out.close_brace(true);
        }

        condition.write_after(config, out);
    }
}
