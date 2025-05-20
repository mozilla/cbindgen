use crate::bindgen::ir::{
    to_known_assoc_constant, ConditionWrite, DeprecatedNoteKind, Documentation, Enum, EnumVariant,
    Field, GenericParams, Item, Literal, OpaqueItem, ReprAlign, Static, Struct, ToCondition, Type,
    Typedef, Union,
};
use crate::bindgen::language_backend::LanguageBackend;
use crate::bindgen::writer::{ListType, SourceWriter};
use crate::bindgen::{cdecl, Bindings, Config, Language};
use crate::bindgen::{DocumentationLength, DocumentationStyle};
use std::io::Write;

pub struct ZigLanguageBackend<'a> {
    config: &'a Config,
}

impl<'a> ZigLanguageBackend<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
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

        // if let Some(bitfield) = f.annotations.atom("bitfield") {
        //     write!(out, ": {}", bitfield.unwrap_or_default());
        // }

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

    fn open_close_namespaces<W: Write>(&mut self, out: &mut SourceWriter<W>, _: bool) {
        out.new_line();
    }
}

impl LanguageBackend for ZigLanguageBackend<'_> {
    fn write_headers<W: Write>(&self, out: &mut SourceWriter<W>, package_version: &str) {
        if self.config.package_version {
            write!(out, "//! Package version: {}", package_version);
            out.new_line();
        }
        if let Some(ref f) = self.config.header {
            out.new_line_if_not_start();
            write!(out, "{}", f);
            out.new_line();
        }
        if self.config.include_version {
            out.new_line_if_not_start();
            write!(
                out,
                "//! Generated with cbindgen:{}",
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
                Language::Zig => {
                    out.write("const std = @import(\"std\");");
                    out.new_line();
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

    fn write_footers<W: Write>(&mut self, _: &mut SourceWriter<W>) {
        //     out.new_line();
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
            // if !inline_tag_field {
            //     out.write("union");
            //     out.open_brace();
            // }

            // Emit fields for all variants with data.
            // e.write_variant_fields(self.config, self, out, inline_tag_field, Self::write_field);

            // Close union of all variants with data, only in the non-inline tag scenario.
            // if !inline_tag_field {
            //     out.close_brace(true);
            // }

            // Emit convenience methods for the struct or enum for the data.
            e.write_derived_functions_data(self.config, self, out, tag_name, Self::write_field);

            // Emit the post_body section, if relevant.
            if let Some(body) = self.config.export.post_body(&e.path) {
                out.new_line();
                out.write_raw_block(body);
            }

            // Close the struct or union opened either at (*) or at (**).
            out.close_brace(false);
            write!(out, ";");
        }

        condition.write_after(self.config, out);
    }

    fn write_struct<W: Write>(&mut self, out: &mut SourceWriter<W>, s: &Struct) {
        if s.is_transparent {
            let typedef = Typedef {
                path: s.path.clone(),
                export_name: s.export_name.to_owned(),
                generic_params: s.generic_params.clone(),
                aliased: s.fields[0].ty.clone(),
                cfg: s.cfg.clone(),
                annotations: s.annotations.clone(),
                documentation: s.documentation.clone(),
            };
            self.write_type_def(out, &typedef);
            for constant in &s.associated_constants {
                out.new_line();
                constant.write(self.config, self, out, Some(s));
            }
            return;
        }

        let condition = s.cfg.to_condition(self.config);
        condition.write_before(self.config, out);

        self.write_documentation(out, &s.documentation);

        if !s.is_enum_variant_body {
            self.write_generic_param(out, &s.generic_params);
        }

        write!(out, "const {} = extern struct", s.export_name);

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

        out.open_brace();

        // Emit the pre_body section, if relevant
        if let Some(body) = self.config.export.pre_body(&s.path) {
            out.write_raw_block(body);
            out.new_line();
        }

        out.write_vertical_source_list(self, &s.fields, ListType::Cap(","), Self::write_field);

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

        out.close_brace(false);
        write!(out, ";");

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

        write!(out, "const {} = extern union", u.export_name);

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
        // todo!("See union tag");
        // if self.config.language != Language::C || self.config.style.generate_tag() {
        //     write!(out, " {}", u.export_name);
        // }

        out.open_brace();

        // Emit the pre_body section, if relevant
        if let Some(body) = self.config.export.pre_body(&u.path) {
            out.write_raw_block(body);
            out.new_line();
        }

        out.write_vertical_source_list(self, &u.fields, ListType::Cap(","), Self::write_field);

        // Emit the post_body section, if relevant
        if let Some(body) = self.config.export.post_body(&u.path) {
            out.new_line();
            out.write_raw_block(body);
        }

        out.close_brace(false);
        write!(out, ";");

        condition.write_after(self.config, out);
    }

    fn write_opaque_item<W: Write>(&mut self, out: &mut SourceWriter<W>, o: &OpaqueItem) {
        let condition = o.cfg.to_condition(self.config);
        condition.write_before(self.config, out);

        self.write_documentation(out, &o.documentation);

        o.generic_params.write_with_default(self, self.config, out);

        // like 'void*' in ziglang (optional null or ptr)
        write!(out, "const {} = ?*anyopaque;", o.export_name());
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
        self.write_documentation(out, &s.documentation);
        out.write("extern ");
        if let Type::Ptr { is_const: true, .. } = s.ty {
        } else if !s.mutable {
            out.write("const ");
        }
        cdecl::write_field(self, out, &s.ty, &s.export_name, self.config);
        out.write(";");
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
            DocumentationStyle::Auto => DocumentationStyle::Zig, // Fallback if `Language` gets extended.
            other => other,
        };

        // Following these documents for style conventions:
        // https://en.wikibooks.org/wiki/C++_Programming/Code/Style_Conventions/Comments
        // https://www.cs.cmu.edu/~410/doc/doxygen.html
        match style {
            DocumentationStyle::Zig => {
                out.write(" ///");
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
                DocumentationStyle::Zig => out.write("///"),
                DocumentationStyle::Auto => unreachable!(), // Auto case should always be covered
            }

            write!(out, "{}", line);
            out.new_line();
        }

        match style {
            DocumentationStyle::Zig => {
                out.write(" ///");
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
                    let path_separator = "_";
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
                write!(out, ":{} = .", export_name);

                write!(out, "{{ ");
                let mut is_first_field = true;
                // In C++, same order as defined is required.
                let ordered_fields = out.bindings().struct_field_names(path);
                for ordered_key in ordered_fields.iter() {
                    if let Some(lit) = fields.get(ordered_key) {
                        if !is_first_field {
                            write!(out, ", ");
                        }
                        is_first_field = false;
                        write!(out, ".{} = ", ordered_key);
                        self.write_literal(out, lit);
                    }
                }
                write!(out, " }}");
            }
        }
    }

    fn write_globals<W: Write>(&mut self, out: &mut SourceWriter<W>, b: &Bindings) {
        // Override default method to open various blocs containing both globals and functions
        // these blocks are closed in [`write_functions`] that is also overridden
        if !b.functions.is_empty() || !b.globals.is_empty() {
            // if b.config.language == Language::Zig {
            //     if let Some(ref using_namespaces) = b.config.using_namespaces {
            //         for namespace in using_namespaces {
            //             out.new_line();
            //             write!(out, "usingnamespace {};", namespace);
            //         }
            //         out.new_line();
            //     }
            // }

            self.write_globals_default(out, b);
        }
    }

    fn write_functions<W: Write>(&mut self, out: &mut SourceWriter<W>, b: &Bindings) {
        // Override default method to close various blocks containing both globals and functions
        // these blocks are opened in [`write_globals`] that is also overridden
        if !b.functions.is_empty() || !b.globals.is_empty() {
            self.write_functions_default(out, b);
        }
    }
}
