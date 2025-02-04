use multimap::MultiMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;

use crate::bindgen::dependencies::Dependencies;
use crate::bindgen::ir::{
    to_known_assoc_constant, ConditionWrite, DeprecatedNoteKind, Documentation, Enum, EnumVariant,
    Field, GenericParams, Item, Literal, OpaqueItem, Path, ReprAlign, Static, Struct, ToCondition,
    Type, Typedef, Union,
};
use crate::bindgen::language_backend::{ItemContainer, LanguageBackend};
use crate::bindgen::rename::IdentifierType;
use crate::bindgen::writer::{ListType, SourceWriter};
use crate::bindgen::{cdecl, Bindings, Config, Language};
use crate::bindgen::{DocumentationLength, DocumentationStyle};

pub struct CLikeLanguageBackend<'a> {
    config: &'a Config,
}

impl<'a> CLikeLanguageBackend<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    /// Resolve the item order by checking inter dependencies, reordering items and adding forward declarations.
    pub fn resolve_order(&self, dependencies: &mut Dependencies) {
        let mut resolver = ResolveOrder::new_preserve_order(self.config, dependencies);
        if let Err(n) = resolver.resolve() {
            warn!("resolve_order: failed with {n} pending items, continuing anyway...");
        }
        for (index, &item_index) in resolver.order.iter().enumerate() {
            let path = resolver.dependencies.order[item_index].deref().path();
            let state = &resolver.states[item_index];
            trace!("DONE {} <- {} {} is {:?}", index, item_index, path, state);
        }
        for &item_index in resolver.pending.iter() {
            let path = resolver.dependencies.order[item_index].deref().path();
            let state = &resolver.states[item_index];
            warn!("PENDING {} {} is {:?}", item_index, path, state);
        }
        resolver.apply();
        for (index, item) in dependencies.order.iter().enumerate() {
            trace!("APPLY {} {}", index, item.deref().path());
        }
    }

    fn write_enum_variant<W: Write>(&mut self, out: &mut SourceWriter<W>, u: &EnumVariant) {
        let condition = u.cfg.to_condition(self.config);

        condition.write_before(self.config, out);

        self.write_documentation(out, &u.documentation);
        write!(out, "{}", u.export_name);
        if let Some(note) = u
            .body
            .annotations()
            .deprecated_note(self.config, DeprecatedNoteKind::EnumVariant)
        {
            write!(out, " {}", note);
        }
        if let Some(discriminant) = &u.discriminant {
            out.write(" = ");

            self.write_literal(out, discriminant);
        }
        out.write(",");
        condition.write_after(self.config, out);
    }

    fn write_field<W: Write>(&mut self, out: &mut SourceWriter<W>, f: &Field) {
        let condition = f.cfg.to_condition(self.config);
        condition.write_before(self.config, out);

        self.write_documentation(out, &f.documentation);
        cdecl::write_field(self, out, &f.ty, &f.name, self.config);

        if let Some(bitfield) = f.annotations.atom("bitfield") {
            write!(out, ": {}", bitfield.unwrap_or_default());
        }

        condition.write_after(self.config, out);
        // FIXME(#634): `write_vertical_source_list` should support
        // configuring list elements natively. For now we print a newline
        // here to avoid printing `#endif;` with semicolon.
        if condition.is_some() {
            out.new_line();
        }
    }

    fn write_generic_param<W: Write>(&mut self, out: &mut SourceWriter<W>, g: &GenericParams) {
        g.write_internal(self, self.config, out, false);
    }

    fn open_close_namespaces<W: Write>(&mut self, out: &mut SourceWriter<W>, open: bool) {
        let mut namespaces =
            if self.config.language != Language::Cxx && !self.config.cpp_compatible_c() {
                vec![]
            } else {
                let mut ret = vec![];
                if let Some(ref namespace) = self.config.namespace {
                    ret.push(&**namespace);
                }
                if let Some(ref namespaces) = self.config.namespaces {
                    for namespace in namespaces {
                        ret.push(&**namespace);
                    }
                }
                ret
            };

        if namespaces.is_empty() {
            return;
        }

        if !open {
            namespaces.reverse();
        }

        if self.config.cpp_compatible_c() {
            out.new_line_if_not_start();
            out.write("#ifdef __cplusplus");
        }

        for namespace in namespaces {
            out.new_line();
            if open {
                write!(out, "namespace {} {{", namespace)
            } else {
                write!(out, "}}  // namespace {}", namespace)
            }
        }

        out.new_line();
        if self.config.cpp_compatible_c() {
            out.write("#endif  // __cplusplus");
            out.new_line();
        }
    }

    fn generate_typedef(&self) -> bool {
        self.config.language == Language::C && self.config.style.generate_typedef()
    }

    fn write_derived_cpp_ops<W: Write>(&mut self, out: &mut SourceWriter<W>, s: &Struct) {
        let mut wrote_start_newline = false;

        if self.config.structure.derive_constructor(&s.annotations) && !s.fields.is_empty() {
            if !wrote_start_newline {
                wrote_start_newline = true;
                out.new_line();
            }

            out.new_line();

            let renamed_fields: Vec<_> = s
                .fields
                .iter()
                .map(|field| {
                    self.config
                        .function
                        .rename_args
                        .apply(&field.name, IdentifierType::FunctionArg)
                        .into_owned()
                })
                .collect();
            write!(out, "{}(", s.export_name());
            let vec: Vec<_> = s
                .fields
                .iter()
                .zip(&renamed_fields)
                .map(|(field, renamed)| {
                    Field::from_name_and_type(
                        // const-ref args to constructor
                        format!("const& {}", renamed),
                        field.ty.clone(),
                    )
                })
                .collect();
            out.write_vertical_source_list(self, &vec[..], ListType::Join(","), Self::write_field);
            write!(out, ")");
            out.new_line();
            write!(out, "  : ");
            let vec: Vec<_> = s
                .fields
                .iter()
                .zip(&renamed_fields)
                .map(|(field, renamed)| format!("{}({})", field.name, renamed))
                .collect();
            out.write_vertical_source_list(self, &vec[..], ListType::Join(","), |_, out, s| {
                write!(out, "{}", s)
            });
            out.new_line();
            write!(out, "{{}}");
            out.new_line();
        }

        let other = self
            .config
            .function
            .rename_args
            .apply("other", IdentifierType::FunctionArg);

        if s.annotations
            .bool("internal-derive-bitflags")
            .unwrap_or(false)
        {
            assert_eq!(s.fields.len(), 1);
            let bits = &s.fields[0].name;
            if !wrote_start_newline {
                wrote_start_newline = true;
                out.new_line();
            }
            let constexpr_prefix = if self.config.constant.allow_constexpr {
                "constexpr "
            } else {
                ""
            };

            out.new_line();
            write!(out, "{}explicit operator bool() const", constexpr_prefix);
            out.open_brace();
            write!(out, "return !!{bits};");
            out.close_brace(false);

            out.new_line();
            write!(
                out,
                "{}{} operator~() const",
                constexpr_prefix,
                s.export_name()
            );
            out.open_brace();
            write!(
                out,
                "return {} {{ static_cast<decltype({bits})>(~{bits}) }};",
                s.export_name()
            );
            out.close_brace(false);
            s.emit_bitflags_binop(constexpr_prefix, '|', &other, out);
            s.emit_bitflags_binop(constexpr_prefix, '&', &other, out);
            s.emit_bitflags_binop(constexpr_prefix, '^', &other, out);
        }

        // Generate a serializer function that allows dumping this struct
        // to an std::ostream. It's defined as a friend function inside the
        // struct definition, and doesn't need the `inline` keyword even
        // though it's implemented right in the generated header file.
        if self.config.structure.derive_ostream(&s.annotations) {
            if !wrote_start_newline {
                wrote_start_newline = true;
                out.new_line();
            }

            out.new_line();
            let stream = self
                .config
                .function
                .rename_args
                .apply("stream", IdentifierType::FunctionArg);
            let instance = self
                .config
                .function
                .rename_args
                .apply("instance", IdentifierType::FunctionArg);
            write!(
                out,
                "friend std::ostream& operator<<(std::ostream& {}, const {}& {})",
                stream,
                s.export_name(),
                instance,
            );
            out.open_brace();
            write!(out, "return {} << \"{{ \"", stream);
            let vec: Vec<_> = s
                .fields
                .iter()
                .map(|x| format!(" << \"{}=\" << {}.{}", x.name, instance, x.name))
                .collect();
            out.write_vertical_source_list(
                self,
                &vec[..],
                ListType::Join(" << \", \""),
                |_, out, s| write!(out, "{}", s),
            );
            out.write(" << \" }\";");
            out.close_brace(false);
        }

        let skip_fields = s.has_tag_field as usize;

        macro_rules! emit_op {
            ($op_name:expr, $op:expr, $conjuc:expr) => {{
                if !wrote_start_newline {
                    #[allow(unused_assignments)]
                    {
                        wrote_start_newline = true;
                    }
                    out.new_line();
                }

                out.new_line();

                if let Some(Some(attrs)) = s.annotations.atom(concat!($op_name, "-attributes")) {
                    write!(out, "{} ", attrs);
                }

                write!(
                    out,
                    "bool operator{}(const {}& {}) const",
                    $op,
                    s.export_name(),
                    other
                );
                out.open_brace();
                out.write("return ");
                let vec: Vec<_> = s
                    .fields
                    .iter()
                    .skip(skip_fields)
                    .map(|field| format!("{} {} {}.{}", field.name, $op, other, field.name))
                    .collect();
                out.write_vertical_source_list(
                    self,
                    &vec[..],
                    ListType::Join(&format!(" {}", $conjuc)),
                    |_, out, s| write!(out, "{}", s),
                );
                out.write(";");
                out.close_brace(false);
            }};
        }

        if self.config.structure.derive_eq(&s.annotations) && s.can_derive_eq() {
            emit_op!("eq", "==", "&&");
        }
        if self.config.structure.derive_neq(&s.annotations) && s.can_derive_eq() {
            emit_op!("neq", "!=", "||");
        }
        if self.config.structure.derive_lt(&s.annotations)
            && s.fields.len() == 1
            && s.fields[0].ty.can_cmp_order()
        {
            emit_op!("lt", "<", "&&");
        }
        if self.config.structure.derive_lte(&s.annotations)
            && s.fields.len() == 1
            && s.fields[0].ty.can_cmp_order()
        {
            emit_op!("lte", "<=", "&&");
        }
        if self.config.structure.derive_gt(&s.annotations)
            && s.fields.len() == 1
            && s.fields[0].ty.can_cmp_order()
        {
            emit_op!("gt", ">", "&&");
        }
        if self.config.structure.derive_gte(&s.annotations)
            && s.fields.len() == 1
            && s.fields[0].ty.can_cmp_order()
        {
            emit_op!("gte", ">=", "&&");
        }
    }
}

impl LanguageBackend for CLikeLanguageBackend<'_> {
    fn write_headers<W: Write>(&self, out: &mut SourceWriter<W>, package_version: &str) {
        if self.config.package_version {
            write!(out, "/* Package version: {} */", package_version);
            out.new_line();
        }
        if let Some(ref f) = self.config.header {
            out.new_line_if_not_start();
            write!(out, "{}", f);
            out.new_line();
        }
        if let Some(f) = self.config.include_guard() {
            out.new_line_if_not_start();
            write!(out, "#ifndef {}", f);
            out.new_line();
            write!(out, "#define {}", f);
            out.new_line();
        }
        if self.config.pragma_once {
            out.new_line_if_not_start();
            write!(out, "#pragma once");
            out.new_line();
        }
        if self.config.include_version {
            out.new_line_if_not_start();
            write!(
                out,
                "/* Generated with cbindgen:{} */",
                crate::bindgen::config::VERSION
            );
            out.new_line();
        }
        if let Some(ref f) = self.config.autogen_warning {
            out.new_line_if_not_start();
            write!(out, "{}", f);
            out.new_line();
        }

        if self.config.no_includes
            && self.config.sys_includes().is_empty()
            && self.config.includes().is_empty()
            && self.config.after_includes.is_none()
        {
            return;
        }

        out.new_line_if_not_start();

        if !self.config.no_includes {
            match self.config.language {
                Language::C => {
                    out.write("#include <stdarg.h>");
                    out.new_line();
                    out.write("#include <stdbool.h>");
                    out.new_line();
                    if self.config.usize_is_size_t {
                        out.write("#include <stddef.h>");
                        out.new_line();
                    }
                    out.write("#include <stdint.h>");
                    out.new_line();
                    out.write("#include <stdlib.h>");
                    out.new_line();
                }
                Language::Cxx => {
                    out.write("#include <cstdarg>");
                    out.new_line();
                    if self.config.usize_is_size_t {
                        out.write("#include <cstddef>");
                        out.new_line();
                    }
                    out.write("#include <cstdint>");
                    out.new_line();
                    out.write("#include <cstdlib>");
                    out.new_line();
                    out.write("#include <ostream>");
                    out.new_line();
                    out.write("#include <new>");
                    out.new_line();
                    if self.config.enumeration.cast_assert_name.is_none()
                        && (self.config.enumeration.derive_mut_casts
                            || self.config.enumeration.derive_const_casts)
                    {
                        out.write("#include <cassert>");
                        out.new_line();
                    }
                }
                _ => {}
            }
        }

        for include in self.config.sys_includes() {
            write!(out, "#include <{}>", include);
            out.new_line();
        }

        for include in self.config.includes() {
            write!(out, "#include \"{}\"", include);
            out.new_line();
        }

        if let Some(ref line) = self.config.after_includes {
            write!(out, "{}", line);
            out.new_line();
        }
    }

    fn open_namespaces<W: Write>(&mut self, out: &mut SourceWriter<W>) {
        self.open_close_namespaces(out, true);
    }

    fn close_namespaces<W: Write>(&mut self, out: &mut SourceWriter<W>) {
        self.open_close_namespaces(out, false)
    }

    fn write_footers<W: Write>(&mut self, out: &mut SourceWriter<W>) {
        if let Some(f) = self.config.include_guard() {
            out.new_line_if_not_start();
            if self.config.language == Language::C {
                write!(out, "#endif  /* {} */", f);
            } else {
                write!(out, "#endif  // {}", f);
            }
            out.new_line();
        }
    }

    fn write_enum<W: Write>(&mut self, out: &mut SourceWriter<W>, e: &Enum) {
        let size = e.repr.ty.map(|ty| ty.to_primitive().to_repr_c(self.config));
        let has_data = e.tag.is_some();
        let inline_tag_field = Enum::inline_tag_field(&e.repr);
        let tag_name = e.tag_name();

        let condition = e.cfg.to_condition(self.config);
        condition.write_before(self.config, out);

        self.write_documentation(out, &e.documentation);
        self.write_generic_param(out, &e.generic_params);

        // If the enum has data, we need to emit a struct or union for the data
        // and enum for the tag. C++ supports nested type definitions, so we open
        // the struct or union here and define the tag enum inside it (*).
        if has_data && self.config.language == Language::Cxx {
            e.open_struct_or_union(self.config, out, inline_tag_field);
        }

        // Emit the tag enum and everything related to it.
        e.write_tag_enum(self.config, self, out, size, Self::write_enum_variant);

        // If the enum has data, we need to emit structs for the variants and gather them together.
        if has_data {
            e.write_variant_defs(self.config, self, out);
            out.new_line();
            out.new_line();

            // Open the struct or union for the data (**), gathering all the variants with data
            // together, unless it's C++, then we have already opened that struct/union at (*) and
            // are currently inside it.
            if self.config.language != Language::Cxx {
                e.open_struct_or_union(self.config, out, inline_tag_field);
            }

            // Emit tag field that is separate from all variants.
            e.write_tag_field(self.config, out, size, inline_tag_field, tag_name);
            out.new_line();

            // Open union of all variants with data, only in the non-inline tag scenario.
            if !inline_tag_field {
                out.write("union");
                out.open_brace();
            }

            // Emit fields for all variants with data.
            e.write_variant_fields(self.config, self, out, inline_tag_field, Self::write_field);

            // Close union of all variants with data, only in the non-inline tag scenario.
            if !inline_tag_field {
                out.close_brace(true);
            }

            // Emit convenience methods for the struct or enum for the data.
            e.write_derived_functions_data(self.config, self, out, tag_name, Self::write_field);

            // Emit the post_body section, if relevant.
            if let Some(body) = self.config.export.post_body(&e.path) {
                out.new_line();
                out.write_raw_block(body);
            }

            // Close the struct or union opened either at (*) or at (**).
            if self.generate_typedef() {
                out.close_brace(false);
                write!(out, " {};", e.export_name);
            } else {
                out.close_brace(true);
            }
        }

        condition.write_after(self.config, out);
    }

    fn write_struct<W: Write>(&mut self, out: &mut SourceWriter<W>, s: &Struct) {
        let condition = s.cfg.to_condition(self.config);
        condition.write_before(self.config, out);

        self.write_documentation(out, &s.documentation);

        if !s.is_enum_variant_body {
            self.write_generic_param(out, &s.generic_params);
        }

        // The following results in
        // C++ or C with Tag as style:
        //   struct Name {
        // C with Type only style:
        //   typedef struct {
        // C with Both as style:
        //   typedef struct Name {
        if self.generate_typedef() {
            out.write("typedef ");
        }

        out.write("struct");

        if let Some(align) = s.alignment {
            match align {
                ReprAlign::Packed => {
                    if let Some(ref anno) = self.config.layout.packed {
                        write!(out, " {}", anno);
                    }
                }
                ReprAlign::Align(n) => {
                    if let Some(ref anno) = self.config.layout.aligned_n {
                        write!(out, " {}({})", anno, n);
                    }
                }
            }
        }

        if s.annotations.must_use(self.config) {
            if let Some(ref anno) = self.config.structure.must_use {
                write!(out, " {}", anno);
            }
        }

        if let Some(note) = s
            .annotations
            .deprecated_note(self.config, DeprecatedNoteKind::Struct)
        {
            write!(out, " {}", note);
        }

        if self.config.language != Language::C || self.config.style.generate_tag() {
            write!(out, " {}", s.export_name());
        }

        out.open_brace();

        // Emit the pre_body section, if relevant
        if let Some(body) = self.config.export.pre_body(&s.path) {
            out.write_raw_block(body);
            out.new_line();
        }

        out.write_vertical_source_list(self, &s.fields, ListType::Cap(";"), Self::write_field);

        if self.config.language == Language::Cxx {
            self.write_derived_cpp_ops(out, s);
        }

        // Emit the post_body section, if relevant
        if let Some(body) = self.config.export.post_body(&s.path) {
            out.new_line();
            out.write_raw_block(body);
        }

        if self.config.language == Language::Cxx
            && self.config.structure.associated_constants_in_body
            && self.config.constant.allow_static_const
        {
            for constant in &s.associated_constants {
                out.new_line();
                constant.write_declaration(self.config, self, out, s);
            }
        }

        if self.generate_typedef() {
            out.close_brace(false);
            write!(out, " {};", s.export_name());
        } else {
            out.close_brace(true);
        }

        for constant in &s.associated_constants {
            out.new_line();
            constant.write(self.config, self, out, Some(s));
        }

        condition.write_after(self.config, out);
    }

    fn write_union<W: Write>(&mut self, out: &mut SourceWriter<W>, u: &Union) {
        let condition = u.cfg.to_condition(self.config);
        condition.write_before(self.config, out);

        self.write_documentation(out, &u.documentation);

        self.write_generic_param(out, &u.generic_params);

        // The following results in
        // C++ or C with Tag as style:
        //   union Name {
        // C with Type only style:
        //   typedef union {
        // C with Both as style:
        //   typedef union Name {
        if self.generate_typedef() {
            out.write("typedef ");
        }

        out.write("union");

        if let Some(align) = u.alignment {
            match align {
                ReprAlign::Packed => {
                    if let Some(ref anno) = self.config.layout.packed {
                        write!(out, " {}", anno);
                    }
                }
                ReprAlign::Align(n) => {
                    if let Some(ref anno) = self.config.layout.aligned_n {
                        write!(out, " {}({})", anno, n);
                    }
                }
            }
        }

        if self.config.language != Language::C || self.config.style.generate_tag() {
            write!(out, " {}", u.export_name);
        }

        out.open_brace();

        // Emit the pre_body section, if relevant
        if let Some(body) = self.config.export.pre_body(&u.path) {
            out.write_raw_block(body);
            out.new_line();
        }

        out.write_vertical_source_list(self, &u.fields, ListType::Cap(";"), Self::write_field);

        // Emit the post_body section, if relevant
        if let Some(body) = self.config.export.post_body(&u.path) {
            out.new_line();
            out.write_raw_block(body);
        }

        if self.generate_typedef() {
            out.close_brace(false);
            write!(out, " {};", u.export_name);
        } else {
            out.close_brace(true);
        }

        condition.write_after(self.config, out);
    }

    fn write_opaque_item<W: Write>(&mut self, out: &mut SourceWriter<W>, o: &OpaqueItem) {
        let condition = o.cfg.to_condition(self.config);
        condition.write_before(self.config, out);

        self.write_documentation(out, &o.documentation);

        o.generic_params.write_with_default(self, self.config, out);

        if self.generate_typedef() {
            write!(
                out,
                "typedef struct {} {};",
                o.export_name(),
                o.export_name()
            );
        } else {
            write!(out, "struct {};", o.export_name());
        }

        condition.write_after(self.config, out);
    }

    fn write_type_def<W: Write>(&mut self, out: &mut SourceWriter<W>, t: &Typedef) {
        let condition = t.cfg.to_condition(self.config);
        condition.write_before(self.config, out);

        self.write_documentation(out, &t.documentation);

        self.write_generic_param(out, &t.generic_params);

        if self.config.language == Language::Cxx {
            write!(out, "using {} = ", t.export_name());
            self.write_type(out, &t.aliased);
        } else {
            write!(out, "{} ", self.config.language.typedef());
            self.write_field(
                out,
                &Field::from_name_and_type(t.export_name().to_owned(), t.aliased.clone()),
            );
        }

        out.write(";");

        condition.write_after(self.config, out);
    }

    fn write_static<W: Write>(&mut self, out: &mut SourceWriter<W>, s: &Static) {
        let condition = s.cfg.to_condition(self.config);
        condition.write_before(self.config, out);

        self.write_documentation(out, &s.documentation);
        out.write("extern ");
        if let Type::Ptr { is_const: true, .. } = s.ty {
        } else if !s.mutable {
            out.write("const ");
        }
        cdecl::write_field(self, out, &s.ty, &s.export_name, self.config);
        out.write(";");

        condition.write_after(self.config, out);
    }

    fn write_type<W: Write>(&mut self, out: &mut SourceWriter<W>, t: &Type) {
        cdecl::write_type(self, out, t, self.config);
    }

    fn write_documentation<W: Write>(&mut self, out: &mut SourceWriter<W>, d: &Documentation) {
        if d.doc_comment.is_empty() || !self.config.documentation {
            return;
        }

        let end = match self.config.documentation_length {
            DocumentationLength::Short => 1,
            DocumentationLength::Full => d.doc_comment.len(),
        };

        let style = match self.config.documentation_style {
            DocumentationStyle::Auto if self.config.language == Language::C => {
                DocumentationStyle::Doxy
            }
            DocumentationStyle::Auto if self.config.language == Language::Cxx => {
                DocumentationStyle::Cxx
            }
            DocumentationStyle::Auto => DocumentationStyle::C, // Fallback if `Language` gets extended.
            other => other,
        };

        // Following these documents for style conventions:
        // https://en.wikibooks.org/wiki/C++_Programming/Code/Style_Conventions/Comments
        // https://www.cs.cmu.edu/~410/doc/doxygen.html
        match style {
            DocumentationStyle::C => {
                out.write("/*");
                out.new_line();
            }

            DocumentationStyle::Doxy => {
                out.write("/**");
                out.new_line();
            }

            _ => (),
        }

        for line in &d.doc_comment[..end] {
            match style {
                DocumentationStyle::C => out.write(""),
                DocumentationStyle::Doxy => out.write(" *"),
                DocumentationStyle::C99 => out.write("//"),
                DocumentationStyle::Cxx => out.write("///"),
                DocumentationStyle::Auto => unreachable!(), // Auto case should always be covered
            }

            write!(out, "{}", line);
            out.new_line();
        }

        match style {
            DocumentationStyle::C => {
                out.write(" */");
                out.new_line();
            }

            DocumentationStyle::Doxy => {
                out.write(" */");
                out.new_line();
            }

            _ => (),
        }
    }

    fn write_literal<W: Write>(&mut self, out: &mut SourceWriter<W>, l: &Literal) {
        match l {
            Literal::Expr(v) => write!(out, "{}", v),
            Literal::Path {
                ref associated_to,
                ref name,
            } => {
                if let Some((ref path, ref export_name)) = associated_to {
                    if let Some(known) = to_known_assoc_constant(path, name) {
                        return write!(out, "{}", known);
                    }
                    let path_separator = if self.config.language == Language::C {
                        "_"
                    } else if self.config.structure.associated_constants_in_body {
                        "::"
                    } else {
                        "_"
                    };
                    write!(out, "{}{}", export_name, path_separator)
                }
                write!(out, "{}", name)
            }
            Literal::FieldAccess {
                ref base,
                ref field,
            } => {
                write!(out, "(");
                self.write_literal(out, base);
                write!(out, ").{}", field);
            }
            Literal::PostfixUnaryOp { op, ref value } => {
                write!(out, "{}", op);
                self.write_literal(out, value);
            }
            Literal::BinOp {
                ref left,
                op,
                ref right,
            } => {
                write!(out, "(");
                self.write_literal(out, left);
                write!(out, " {} ", op);
                self.write_literal(out, right);
                write!(out, ")");
            }
            Literal::Cast { ref ty, ref value } => {
                out.write("(");
                self.write_type(out, ty);
                out.write(")");
                self.write_literal(out, value);
            }
            Literal::Struct {
                export_name,
                fields,
                path,
            } => {
                let allow_constexpr = self.config.constant.allow_constexpr && l.can_be_constexpr();
                let is_constexpr = self.config.language == Language::Cxx
                    && (self.config.constant.allow_static_const || allow_constexpr);
                if self.config.language == Language::C {
                    write!(out, "({})", export_name);
                } else {
                    write!(out, "{}", export_name);
                }

                write!(out, "{{");
                if is_constexpr {
                    out.push_tab();
                } else {
                    write!(out, " ");
                }
                // In C++, same order as defined is required.
                let ordered_fields = out.bindings().struct_field_names(path);
                for (i, ordered_key) in ordered_fields.iter().enumerate() {
                    if let Some(lit) = fields.get(ordered_key) {
                        if is_constexpr {
                            out.new_line();

                            // TODO: Some C++ versions (c++20?) now support designated
                            // initializers, consider generating them.
                            write!(out, "/* .{} = */ ", ordered_key);
                            self.write_literal(out, lit);
                            if i + 1 != ordered_fields.len() {
                                write!(out, ",");
                                if !is_constexpr {
                                    write!(out, " ");
                                }
                            }
                        } else {
                            if i > 0 {
                                write!(out, ", ");
                            }

                            if self.config.language == Language::Cxx {
                                // TODO: Some C++ versions (c++20?) now support designated
                                // initializers, consider generating them.
                                write!(out, "/* .{} = */ ", ordered_key);
                            } else {
                                write!(out, ".{} = ", ordered_key);
                            }
                            self.write_literal(out, lit);
                        }
                    }
                }
                if is_constexpr {
                    out.pop_tab();
                    out.new_line();
                } else {
                    write!(out, " ");
                }
                write!(out, "}}");
            }
        }
    }

    fn write_globals<W: Write>(&mut self, out: &mut SourceWriter<W>, b: &Bindings) {
        // Override default method to open various blocs containing both globals and functions
        // these blocks are closed in [`write_functions`] that is also overridden
        if !b.functions.is_empty() || !b.globals.is_empty() {
            if b.config.cpp_compatible_c() {
                out.new_line_if_not_start();
                out.write("#ifdef __cplusplus");
            }

            if b.config.language == Language::Cxx {
                if let Some(ref using_namespaces) = b.config.using_namespaces {
                    for namespace in using_namespaces {
                        out.new_line();
                        write!(out, "using namespace {};", namespace);
                    }
                    out.new_line();
                }
            }

            if b.config.language == Language::Cxx || b.config.cpp_compatible_c() {
                out.new_line();
                out.write("extern \"C\" {");
                out.new_line();
            }

            if b.config.cpp_compatible_c() {
                out.write("#endif // __cplusplus");
                out.new_line();
            }

            self.write_globals_default(out, b);
        }
    }

    fn write_functions<W: Write>(&mut self, out: &mut SourceWriter<W>, b: &Bindings) {
        // Override default method to close various blocks containing both globals and functions
        // these blocks are opened in [`write_globals`] that is also overridden
        if !b.functions.is_empty() || !b.globals.is_empty() {
            self.write_functions_default(out, b);

            if b.config.cpp_compatible_c() {
                out.new_line();
                out.write("#ifdef __cplusplus");
            }

            if b.config.language == Language::Cxx || b.config.cpp_compatible_c() {
                out.new_line();
                out.write("}  // extern \"C\"");
                out.new_line();
            }

            if b.config.cpp_compatible_c() {
                out.write("#endif  // __cplusplus");
                out.new_line();
            }
        }
    }

    fn write_declaration<W: Write>(&mut self, out: &mut SourceWriter<W>, d: &ItemContainer) {
        let some_condition = d.deref().cfg().map(|cfg| cfg.to_condition(self.config));
        if let Some(condition) = &some_condition {
            condition.write_before(self.config, out);
        }

        match d {
            ItemContainer::Struct(ref s) => write!(out, "struct {};", s.export_name),
            ItemContainer::Union(ref u) => write!(out, "union {};", u.export_name),
            ItemContainer::Enum(ref e) => {
                if self.config.language == Language::C {
                    write!(out, "enum {};", e.export_name);
                } else {
                    unreachable!();
                }
            }
            _ => unreachable!(),
        }

        if let Some(condition) = &some_condition {
            condition.write_after(self.config, out);
        }
    }
}

/// State of an item.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ResolveOrderState {
    Pending,            // cannot be used
    Declaration(usize), // declared, can be used in pointers and references
    Alias(Path),        // declared, defined when path is defined
    Defined,            // defined (and declared), can be used
}

/// What the resolver wants.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ResolveOrderWants {
    Declaration(Path),
    Definition(Path),
}
impl ResolveOrderWants {
    pub fn path(&self) -> &Path {
        match self {
            Self::Declaration(ref path) => path,
            Self::Definition(ref path) => path,
        }
    }
}

/// Resolve the order of dependencies.
///
/// Can try to preserve the original order.
/// Assumes all unknown paths are defined.
/// Multiple items can have the same path.
///
/// Algorithm:
/// - if no pending items: done
/// - if countdown is over or no item: give up
/// - if no unresolved dependencies: alias or define item, reset countdown or retry pending
/// - if can declare dependencies: declare dependencies, alias or define item, reset countdown or retry pending
/// - next item, decrease countdown
pub struct ResolveOrder<'a> {
    /// Configuration.
    pub config: &'a Config,
    /// Contains items that need to be resolved.
    pub dependencies: &'a mut Dependencies,
    /// Location of each path.
    pub paths2indexes: MultiMap<Path, usize>,
    /// Contains the state of each item.
    pub states: Vec<ResolveOrderState>,
    /// The pending order of items.
    pub pending: Vec<usize>,
    /// Current pending item being processed.
    pub current: usize,
    /// When to give up.
    pub countdown: Option<usize>,
    /// The resolved order of items.
    pub order: Vec<usize>,
}

impl<'a> ResolveOrder<'a> {
    /// Create a new resolver capable of changing the order of the dependencies.
    ///
    /// Does not try to preserve the original order.
    pub fn new(config: &'a Config, dependencies: &'a mut Dependencies) -> Self {
        let num_items = dependencies.order.len();
        let mut paths2indexes: MultiMap<Path, usize> = MultiMap::new();
        for (item_index, item) in dependencies.order.iter().enumerate() {
            let path = item.deref().path().clone();
            paths2indexes.insert(path, item_index);
        }
        let states = vec![ResolveOrderState::Pending; num_items];
        let pending: Vec<usize> = (0..num_items).collect();
        Self {
            config,
            dependencies,
            paths2indexes,
            states,
            pending,
            current: 0,
            countdown: Some(num_items),
            order: Vec::new(),
        }
    }

    /// Create a new resolver capable of changing the order of the dependencies.
    ///
    /// Tries to preserve the original order.
    pub fn new_preserve_order(config: &'a Config, dependencies: &'a mut Dependencies) -> Self {
        let mut resolver = Self::new(config, dependencies);
        resolver.countdown = None;
        resolver
    }

    /// Apply the computed order to the dependencies, consuming the resolver.
    ///
    /// Pending dependencies are assume resolved.
    pub fn apply(mut self) {
        self.order.extend(self.pending);
        sort_by_order(&mut self.dependencies.order, self.order);
    }

    /// Resolve dependencies by computing a new order of items.
    ///
    /// Can add item declarations.
    ///
    /// Returns `Ok(())` when done.
    /// Returns `Err(n)` when it gives up with `n` pending items.
    pub fn resolve(&mut self) -> Result<(), usize> {
        loop {
            if self.pending.is_empty() {
                trace!("resolve: done");
                return Ok(());
            }
            let give_up = if let Some(n) = self.countdown {
                if self.current == self.pending.len() {
                    self.current = 0;
                }
                n == 0
            } else {
                self.current == self.pending.len()
            };
            if give_up {
                let n = self.pending.len();
                trace!("resolve: give up with {n} pending items");
                return Err(n);
            }
            trace!(
                "resolve: trying index {} of {}...",
                self.current,
                self.pending.len()
            );
            let item_index = self.pending[self.current];
            let unresolved = self.unresolved_of(item_index);
            if unresolved.is_empty() {
                self.alias_or_define(item_index);
                if self.countdown.is_some() {
                    self.countdown = Some(self.pending.len());
                } else {
                    self.current = 0;
                }
                continue;
            }
            if let Some(declarations) = self.declarations(&unresolved) {
                for declaration in declarations.into_iter() {
                    self.declare(declaration);
                }
                self.alias_or_define(item_index);
                if self.countdown.is_some() {
                    self.countdown = Some(self.pending.len());
                } else {
                    self.current = 0;
                }
                continue;
            }
            // try next item
            trace!(
                "resolve: {} depends on {}",
                self.dependencies.order[item_index].deref().path(),
                self.unresolved_string(&unresolved),
            );
            self.current += 1;
            if let Some(n) = self.countdown {
                self.countdown = Some(n - 1);
            }
        }
    }

    /// Alias an item or define an item.
    pub fn alias_or_define(&mut self, item_index: usize) {
        let path = self.dependencies.order[item_index].deref().path();
        if let Some(target) = self.alias_of(item_index) {
            let is_defined = self
                .paths2indexes
                .get_vec(&target)
                .expect("vec")
                .iter()
                .all(|&index| self.states[index] == ResolveOrderState::Defined);
            if !is_defined {
                trace!("alias_or_define: {path} alias to {target}");
                self.states[item_index] = ResolveOrderState::Alias(target);
                self.done(item_index);
                return;
            }
            trace!("alias_or_define: {target} is defined");
        }
        self.define(item_index);
    }

    /// Define an item.
    pub fn define(&mut self, item_index: usize) {
        let path = self.dependencies.order[item_index].deref().path().clone();
        trace!("define: {path} defined");
        self.states[item_index] = ResolveOrderState::Defined;
        self.done(item_index);

        // also define alias to item
        let mut define_alias_to = vec![path];
        while let Some(defined) = define_alias_to.pop() {
            for (path, indexes) in self.paths2indexes.iter_all() {
                for &index in indexes.iter() {
                    if matches!(self.states[index], ResolveOrderState::Alias(ref target) if target == &defined)
                    {
                        // also define alias to alias
                        define_alias_to.push(path.clone());
                        trace!("define: {path} alias to {defined} defined");
                        self.states[index] = ResolveOrderState::Defined;
                    }
                }
            }
        }
    }

    /// Add item declaration.
    pub fn declare(&mut self, state: ResolveOrderState) {
        if let ResolveOrderState::Declaration(target) = state {
            let pending_item = self.dependencies.order[target].clone();
            let item = ItemContainer::Declaration(Box::new(pending_item));
            let path = item.deref().path().clone();
            trace!("declare: {path} declared");
            let item_index = self.dependencies.order.len();
            self.dependencies.order.push(item);
            self.paths2indexes.insert(path, item_index);
            self.states.push(state);
            self.order.push(item_index);
            assert_eq!(
                self.dependencies.order.len(),
                self.pending.len() + self.order.len()
            );
        } else {
            unreachable!();
        }
    }

    /// Move item from the pending list to the order list.
    pub fn done(&mut self, item_index: usize) {
        self.pending.remove(
            self.pending
                .iter()
                .position(|&index| index == item_index)
                .expect("pending index"),
        );
        self.order.push(item_index);
        assert_eq!(
            self.dependencies.order.len(),
            self.pending.len() + self.order.len()
        );
    }

    /// Get the string of unresolved dependencies.
    pub fn unresolved_string(&self, unresolved: &HashSet<ResolveOrderWants>) -> String {
        let unresolved: Vec<_> = unresolved.iter().map(|want| want.path().name()).collect();
        unresolved.join(",")
    }

    /// Get the unresolved dependencies of an item.
    pub fn unresolved_of(&self, item_index: usize) -> HashSet<ResolveOrderWants> {
        // gather wants
        struct GatherWants {
            what: HashMap<Path, ResolveOrderWants>,
        }
        impl GatherWants {
            fn new() -> Self {
                Self {
                    what: HashMap::new(),
                }
            }
            fn want_declaration_of_path(&mut self, path: &Path) {
                if self.what.contains_key(path) {
                    // already want declaration or definition
                    return;
                }
                trace!("unresolved_of: want declaration of {path}");
                let want = ResolveOrderWants::Declaration(path.clone());
                self.what.insert(path.clone(), want);
            }
            fn want_definition_of_path(&mut self, path: &Path) {
                if matches!(self.what.get(path), Some(ResolveOrderWants::Definition(_))) {
                    // already want definition
                    return;
                }
                let path = path.clone();
                trace!("unresolved_of: want definition of {path}");
                let want = ResolveOrderWants::Definition(path.clone());
                self.what.insert(path, want);
            }
            fn want_declaration_of_type(&mut self, x: &Type) {
                match x {
                    Type::Ptr { ref ty, .. } => {
                        ty.visit_root_paths(|path| {
                            self.want_declaration_of_path(&path);
                        });
                    }
                    Type::Path(ref generic_path) => {
                        self.want_declaration_of_path(generic_path.path());
                    }
                    Type::Primitive(_) => {}
                    Type::Array(ref ty, ..) => {
                        self.want_declaration_of_type(ty);
                    }
                    Type::FuncPtr {
                        ref ret, ref args, ..
                    } => {
                        self.want_declaration_of_type(ret);
                        for (_, ty) in args {
                            self.want_declaration_of_type(ty)
                        }
                    }
                }
            }
            fn want_definition_of_type(&mut self, x: &Type) {
                match x {
                    Type::Ptr { ref ty, .. } => {
                        ty.visit_root_paths(|path| {
                            self.want_declaration_of_path(&path);
                        });
                    }
                    Type::Path(ref generic_path) => {
                        self.want_definition_of_path(&generic_path.path().clone());
                    }
                    Type::Primitive(_) => {}
                    Type::Array(ref ty, ..) => {
                        self.want_definition_of_type(ty);
                    }
                    Type::FuncPtr {
                        ref ret, ref args, ..
                    } => {
                        self.want_declaration_of_type(ret);
                        for (_, ty) in args {
                            self.want_declaration_of_type(ty)
                        }
                    }
                }
            }
        }
        let mut wants = GatherWants::new();
        match &self.dependencies.order[item_index] {
            ItemContainer::Typedef(ref x) => {
                wants.want_declaration_of_type(&x.aliased);
            }
            ItemContainer::Struct(ref x) => {
                for field in x.fields.iter() {
                    wants.want_definition_of_type(&field.ty);
                }
            }
            ItemContainer::Union(ref x) => {
                for field in x.fields.iter() {
                    wants.want_definition_of_type(&field.ty);
                }
            }
            _ => {}
        }

        // resolve wants
        let item_path = self.dependencies.order[item_index].deref().path();
        let mut unresolved: HashSet<ResolveOrderWants> = HashSet::new();
        for (_, want) in wants.what.into_iter() {
            match want {
                ResolveOrderWants::Declaration(ref dependency) => {
                    if !self.is_declared(dependency) {
                        trace!("unresolved_of: {dependency} not declared");
                        unresolved.insert(want);
                    }
                }
                ResolveOrderWants::Definition(ref dependency) => {
                    if !self.is_defined(dependency) {
                        trace!("unresolved_of: {dependency} not defined");
                        unresolved.insert(want);
                    }
                }
            }
        }
        unresolved.retain(|want| {
            if let ResolveOrderWants::Declaration(ref dependency) = want {
                if dependency == item_path {
                    trace!("unresolved_of: {dependency} assume self declared");
                    return false;
                }
            }
            true
        });
        unresolved
    }

    /// Try to get declarations of all unresolved dependencies.
    pub fn declarations(
        &self,
        unresolved: &HashSet<ResolveOrderWants>,
    ) -> Option<Vec<ResolveOrderState>> {
        let mut declarations = Vec::new();
        for want in unresolved.iter() {
            match want {
                ResolveOrderWants::Declaration(ref path) => {
                    for &item_index in self.paths2indexes.get_vec(path).expect("vec").iter() {
                        if let Some(declaration) = self.declaration_of(item_index) {
                            declarations.push(declaration);
                            continue;
                        }
                        trace!("declarations: {path} cannot be declared");
                        return None;
                    }
                }
                ResolveOrderWants::Definition(ref path) => {
                    trace!("declarations: {path} cannot be defined");
                    return None;
                }
            }
        }
        Some(declarations)
    }

    /// Try to get the declaration of an item.
    pub fn declaration_of(&self, item_index: usize) -> Option<ResolveOrderState> {
        let item = &self.dependencies.order[item_index];
        let path = item.deref().path();
        match item {
            ItemContainer::Struct(_) | ItemContainer::Union(_) => {
                if self.will_be_named(item_index) {
                    trace!("declaration_of: {path} can be declared");
                    return Some(ResolveOrderState::Declaration(item_index));
                }
            }
            ItemContainer::Enum(_) => {
                if self.will_be_named(item_index) && self.config.language == Language::C {
                    // C allows, Cxx errors
                    trace!("declaration_of: {path} can be declared");
                    return Some(ResolveOrderState::Declaration(item_index));
                }
            }
            _ => {}
        }
        trace!("declaration_of: {path} cannot be declared");
        None
    }

    /// Try to get the target of an alias item.
    pub fn alias_of(&self, item_index: usize) -> Option<Path> {
        let item = &self.dependencies.order[item_index];
        if let ItemContainer::Typedef(ref x) = item {
            if let Type::Path(ref generic_path) = x.aliased {
                let path = item.deref().path();
                let target = generic_path.path();
                trace!("alias_of: {path} is alias of {target}");
                return Some(target.clone());
            }
        }
        None
    }

    /// Check if a path is declared.
    pub fn is_declared(&self, path: &Path) -> bool {
        if let Some(indexes) = self.paths2indexes.get_vec(path) {
            let is_declared = indexes.iter().all(|&index| {
                match self.states[index] {
                    ResolveOrderState::Pending => {
                        indexes.iter().any(|&declaration_index| {
                            matches!(self.states[declaration_index], ResolveOrderState::Declaration(target) if target == index)
                        })
                    }
                    _ => true,
                }
            });
            if is_declared {
                trace!("is_declared: {path} is declared");
            } else {
                trace!("is_declared: {path} is not declared");
            }
            is_declared
        } else {
            trace!("is_declared: {path} is assume declared");
            true
        }
    }

    /// Check if a path is defined.
    pub fn is_defined(&self, path: &Path) -> bool {
        if let Some(indexes) = self.paths2indexes.get_vec(path) {
            let mut is_defined = false;
            for &index in indexes.iter() {
                match self.states[index] {
                    ResolveOrderState::Pending => {
                        is_defined = false;
                        break;
                    }
                    ResolveOrderState::Declaration(_) => {}
                    ResolveOrderState::Defined => is_defined = true,
                    ResolveOrderState::Alias(ref target) => {
                        // assume path is defined if it is an alias to path
                        is_defined = target == path;
                        if !is_defined {
                            break;
                        }
                    }
                }
            }
            if is_defined {
                trace!("is_defined: {path} is defined");
            } else {
                trace!("is_defined: {path} is not defined");
            }
            is_defined
        } else {
            trace!("is_defined: {path} is assume defined");
            true
        }
    }

    /// Check if the language backend will write names.
    pub fn will_be_named(&self, item_index: usize) -> bool {
        let item = &self.dependencies.order[item_index];
        match item {
            ItemContainer::Struct(_) | ItemContainer::Union(_) => {
                // see docs for codegen option `style="type"`
                self.config.language != Language::C || self.config.style.generate_tag()
            }
            _ => true,
        }
    }
}

/// Put the data values in the target order.
///
/// Panics if the data and the order have different sizes.
pub fn sort_by_order<T>(data: &mut [T], mut order: Vec<usize>) {
    assert_eq!(data.len(), order.len());
    for mut to in 0..order.len() {
        let mut from = order[to];
        if from != to {
            // swap the whole chain of data into place
            order[to] = to;
            while order[from] != from {
                data.swap(from, to);
                to = from;
                from = order[to];
                order[to] = to;
            }
        }
    }
}

#[test]
fn test_sort_by_order() {
    macro_rules! test {
        ($a:literal, $b:literal, $c:literal, $d:literal) => {
            let order = [$a, $b, $c, $d];
            let expect = [
                stringify!($a),
                stringify!($b),
                stringify!($c),
                stringify!($d),
            ];
            let mut data = ["0", "1", "2", "3"];
            sort_by_order(&mut data, order.to_vec());
            assert!(
                expect == data,
                "order {:?} produced {:?}, but {:?} was expected",
                order,
                data,
                expect
            );
        };
    }
    test!(0, 1, 2, 3);
    test!(0, 1, 3, 2);
    test!(0, 2, 1, 3);
    test!(0, 2, 3, 1);
    test!(0, 3, 1, 2);
    test!(0, 3, 2, 1);
    test!(1, 0, 2, 3);
    test!(1, 0, 3, 2);
    test!(1, 2, 0, 3);
    test!(1, 2, 3, 0);
    test!(1, 3, 0, 2);
    test!(1, 3, 2, 0);
    test!(2, 0, 1, 3);
    test!(2, 0, 3, 1);
    test!(2, 1, 0, 3);
    test!(2, 1, 3, 0);
    test!(2, 3, 0, 1);
    test!(2, 3, 1, 0);
    test!(3, 0, 1, 2);
    test!(3, 0, 2, 1);
    test!(3, 1, 0, 2);
    test!(3, 1, 2, 0);
    test!(3, 2, 0, 1);
    test!(3, 2, 1, 0);
}
