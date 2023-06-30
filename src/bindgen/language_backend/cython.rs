use crate::bindgen::ir::{
    to_known_assoc_constant, ConditionWrite, DeprecatedNoteKind, Documentation, Enum, EnumVariant,
    Field, Function, GenericParams, Item, Literal, OpaqueItem, ReprAlign, Static, Struct,
    ToCondition, Type, Typedef, Union,
};
use crate::bindgen::language_backend::{LanguageBackend, NamespaceOperation};
use crate::bindgen::writer::{ListType, Source, SourceWriter};
use crate::bindgen::DocumentationLength;
use crate::bindgen::Layout;
use crate::bindgen::{cdecl, Config};
use std::io::Write;

pub struct CythonLanguageBackend {
    config: Config,
}

impl CythonLanguageBackend {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl LanguageBackend for CythonLanguageBackend {
    fn write_headers<W: Write>(&self, out: &mut SourceWriter<W>) {
        if let Some(ref f) = self.config.header {
            out.new_line_if_not_start();
            write!(out, "{}", f);
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
            && (self.config.cython.cimports.is_empty())
            && self.config.after_includes.is_none()
        {
            return;
        }

        out.new_line_if_not_start();

        if !self.config.no_includes {
            out.write("from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t");
            out.new_line();
            out.write("from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t");
            out.new_line();
            out.write("cdef extern from *");
            out.open_brace();
            out.write("ctypedef bint bool");
            out.new_line();
            out.write("ctypedef struct va_list");
            out.new_line();
            out.close_brace(false);
        }

        for (module, names) in &self.config.cython.cimports {
            write!(out, "from {} cimport {}", module, names.join(", "));
            out.new_line();
        }

        if let Some(ref line) = self.config.after_includes {
            write!(out, "{}", line);
            out.new_line();
        }
    }

    fn open_close_namespaces<W: Write>(&self, op: NamespaceOperation, out: &mut SourceWriter<W>) {
        if op == NamespaceOperation::Open {
            out.new_line();
            let header = self.config.cython.header.as_deref().unwrap_or("*");
            write!(out, "cdef extern from {}", header);
            out.open_brace();
        } else {
            out.close_brace(false);
        }
    }

    fn write_footers<W: Write>(&self, _out: &mut SourceWriter<W>) {}
}

impl Source<CythonLanguageBackend> for EnumVariant {
    fn write<F: Write>(&self, language_backend: &CythonLanguageBackend, out: &mut SourceWriter<F>) {
        self.documentation.write(language_backend, out);
        write!(out, "{}", self.export_name);
        if let Some(discriminant) = &self.discriminant {
            // For extern Cython declarations the enumerator value is ignored,
            // but still useful as documentation, so we write it as a comment.
            out.write(" #");

            out.write(" = ");

            discriminant.write(language_backend, out);
        }
        out.write(",");
    }
}

impl Source<CythonLanguageBackend> for Enum {
    fn write<F: Write>(&self, language_backend: &CythonLanguageBackend, out: &mut SourceWriter<F>) {
        let size = self
            .repr
            .ty
            .map(|ty| ty.to_primitive().to_repr_c(&language_backend.config));
        let has_data = self.tag.is_some();
        let inline_tag_field = Self::inline_tag_field(&self.repr);
        let tag_name = self.tag_name();

        let condition = self.cfg.to_condition(&language_backend.config);
        condition.write_before(&language_backend.config, out);

        self.documentation.write(language_backend, out);
        self.generic_params.write(language_backend, out);

        // Emit the tag enum and everything related to it.
        self.write_tag_enum(
            &language_backend.config,
            language_backend,
            out,
            size,
            has_data,
            tag_name,
        );

        // If the enum has data, we need to emit structs for the variants and gather them together.
        if has_data {
            self.write_variant_defs(&language_backend.config, language_backend, out);
            out.new_line();
            out.new_line();

            self.open_struct_or_union(&language_backend.config, out, inline_tag_field);

            // Emit tag field that is separate from all variants.
            self.write_tag_field(
                &language_backend.config,
                out,
                size,
                inline_tag_field,
                tag_name,
            );
            out.new_line();

            // Emit fields for all variants with data.
            self.write_variant_fields(
                &language_backend.config,
                language_backend,
                out,
                inline_tag_field,
            );

            // Emit the post_body section, if relevant.
            if let Some(body) = language_backend.config.export.post_body(&self.path) {
                out.new_line();
                out.write_raw_block(body);
            }

            out.close_brace(true);
        }

        condition.write_after(&language_backend.config, out);
    }
}

impl Source<CythonLanguageBackend> for Struct {
    fn write<F: Write>(&self, language_backend: &CythonLanguageBackend, out: &mut SourceWriter<F>) {
        if self.is_transparent {
            let typedef = Typedef {
                path: self.path.clone(),
                export_name: self.export_name.to_owned(),
                generic_params: self.generic_params.clone(),
                aliased: self.fields[0].ty.clone(),
                cfg: self.cfg.clone(),
                annotations: self.annotations.clone(),
                documentation: self.documentation.clone(),
            };
            typedef.write(language_backend, out);
            for constant in &self.associated_constants {
                out.new_line();
                constant.write(&language_backend.config, language_backend, out, Some(self));
            }
            return;
        }

        let condition = self.cfg.to_condition(&language_backend.config);
        condition.write_before(&language_backend.config, out);

        self.documentation.write(language_backend, out);

        if !self.is_enum_variant_body {
            self.generic_params.write(language_backend, out);
        }

        out.write(language_backend.config.style.cython_def());

        // Cython extern declarations don't manage layouts, layouts are defined entierly by the
        // corresponding C code. So this `packed` is only for documentation, and missing
        // `aligned(n)` is also not a problem.
        if let Some(align) = self.alignment {
            match align {
                ReprAlign::Packed => out.write("packed "),
                ReprAlign::Align(_) => {} // Not supported
            }
        }

        out.write("struct");

        if self.annotations.must_use(&language_backend.config) {
            if let Some(ref anno) = language_backend.config.structure.must_use {
                write!(out, " {}", anno);
            }
        }

        if let Some(note) = self
            .annotations
            .deprecated_note(&language_backend.config, DeprecatedNoteKind::Struct)
        {
            write!(out, " {}", note);
        }

        write!(out, " {}", self.export_name());

        out.open_brace();

        // Emit the pre_body section, if relevant
        if let Some(body) = language_backend.config.export.pre_body(&self.path) {
            out.write_raw_block(body);
            out.new_line();
        }

        out.write_vertical_source_list(language_backend, &self.fields, ListType::Cap(";"));
        if self.fields.is_empty() {
            out.write("pass");
        }

        // Emit the post_body section, if relevant
        if let Some(body) = language_backend.config.export.post_body(&self.path) {
            out.new_line();
            out.write_raw_block(body);
        }
        out.close_brace(true);

        for constant in &self.associated_constants {
            out.new_line();
            constant.write(&language_backend.config, language_backend, out, Some(self));
        }

        condition.write_after(&language_backend.config, out);
    }
}

impl Source<CythonLanguageBackend> for Union {
    fn write<F: Write>(&self, language_backend: &CythonLanguageBackend, out: &mut SourceWriter<F>) {
        let condition = self.cfg.to_condition(&language_backend.config);
        condition.write_before(&language_backend.config, out);

        self.documentation.write(language_backend, out);

        self.generic_params.write(language_backend, out);

        out.write(language_backend.config.style.cython_def());

        out.write("union");

        write!(out, " {}", self.export_name);

        out.open_brace();

        // Emit the pre_body section, if relevant
        if let Some(body) = language_backend.config.export.pre_body(&self.path) {
            out.write_raw_block(body);
            out.new_line();
        }

        out.write_vertical_source_list(language_backend, &self.fields, ListType::Cap(";"));
        if self.fields.is_empty() {
            out.write("pass");
        }

        // Emit the post_body section, if relevant
        if let Some(body) = language_backend.config.export.post_body(&self.path) {
            out.new_line();
            out.write_raw_block(body);
        }

        out.close_brace(true);

        condition.write_after(&language_backend.config, out);
    }
}

impl Source<CythonLanguageBackend> for OpaqueItem {
    fn write<F: Write>(&self, language_backend: &CythonLanguageBackend, out: &mut SourceWriter<F>) {
        let condition = self.cfg.to_condition(&language_backend.config);
        condition.write_before(&language_backend.config, out);

        self.documentation.write(language_backend, out);

        self.generic_params
            .write_with_default(language_backend, &language_backend.config, out);

        write!(
            out,
            "{}struct {}",
            language_backend.config.style.cython_def(),
            self.export_name()
        );
        out.open_brace();
        out.write("pass");
        out.close_brace(false);

        condition.write_after(&language_backend.config, out);
    }
}

impl Source<CythonLanguageBackend> for Field {
    fn write<F: Write>(&self, language_backend: &CythonLanguageBackend, out: &mut SourceWriter<F>) {
        // Cython doesn't support conditional fields.
        // let condition = self.cfg.to_condition(&language_backend.config);

        self.documentation.write(language_backend, out);
        cdecl::write_field(
            language_backend,
            out,
            &self.ty,
            &self.name,
            &language_backend.config,
        );

        // Cython extern declarations don't manage layouts, layouts are defined entierly by the
        // corresponding C code. So we can omit bitfield sizes which are not supported by Cython.
        // if let Some(bitfield) = self.annotations.atom("bitfield") {
        //
        // }
    }
}

impl Source<CythonLanguageBackend> for GenericParams {
    fn write<F: Write>(&self, _language_backend: &CythonLanguageBackend, _: &mut SourceWriter<F>) {
        // not supported
    }
}

impl Source<CythonLanguageBackend> for Typedef {
    fn write<F: Write>(&self, language_backend: &CythonLanguageBackend, out: &mut SourceWriter<F>) {
        let condition = self.cfg.to_condition(&language_backend.config);
        condition.write_before(&language_backend.config, out);

        self.documentation.write(language_backend, out);

        self.generic_params.write(language_backend, out);

        write!(out, "{} ", language_backend.config.language.typedef());
        Field::from_name_and_type(self.export_name().to_owned(), self.aliased.clone())
            .write(language_backend, out);

        out.write(";");

        condition.write_after(&language_backend.config, out);
    }
}

impl Source<CythonLanguageBackend> for Static {
    fn write<F: Write>(&self, language_backend: &CythonLanguageBackend, out: &mut SourceWriter<F>) {
        out.write("extern ");
        if let Type::Ptr { is_const: true, .. } = self.ty {
        } else if !self.mutable {
            out.write("const ");
        }
        cdecl::write_field(
            language_backend,
            out,
            &self.ty,
            &self.export_name,
            &language_backend.config,
        );
        out.write(";");
    }
}

impl Source<CythonLanguageBackend> for Function {
    fn write<F: Write>(&self, language_backend: &CythonLanguageBackend, out: &mut SourceWriter<F>) {
        fn write_1<W: Write>(
            func: &Function,
            language_backend: &CythonLanguageBackend,
            out: &mut SourceWriter<W>,
        ) {
            let prefix = language_backend.config.function.prefix(&func.annotations);
            let postfix = language_backend.config.function.postfix(&func.annotations);

            let condition = func.cfg.to_condition(&language_backend.config);
            condition.write_before(&language_backend.config, out);

            func.documentation.write(language_backend, out);

            if func.extern_decl {
                out.write("extern ");
            } else {
                if let Some(ref prefix) = prefix {
                    write!(out, "{} ", prefix);
                }
                if func.annotations.must_use(&language_backend.config) {
                    if let Some(ref anno) = language_backend.config.function.must_use {
                        write!(out, "{} ", anno);
                    }
                }
                if let Some(note) = func
                    .annotations
                    .deprecated_note(&language_backend.config, DeprecatedNoteKind::Function)
                {
                    write!(out, "{} ", note);
                }
            }
            cdecl::write_func(
                language_backend,
                out,
                func,
                Layout::Horizontal,
                &language_backend.config,
            );

            if !func.extern_decl {
                if let Some(ref postfix) = postfix {
                    write!(out, " {}", postfix);
                }
            }

            if let Some(ref swift_name_macro) = language_backend.config.function.swift_name_macro {
                if let Some(swift_name) = func.swift_name(&language_backend.config) {
                    write!(out, " {}({})", swift_name_macro, swift_name);
                }
            }

            out.write(";");

            condition.write_after(&language_backend.config, out);
        }

        fn write_2<W: Write>(
            func: &Function,
            language_backend: &CythonLanguageBackend,
            out: &mut SourceWriter<W>,
        ) {
            let prefix = language_backend.config.function.prefix(&func.annotations);
            let postfix = language_backend.config.function.postfix(&func.annotations);

            let condition = func.cfg.to_condition(&language_backend.config);

            condition.write_before(&language_backend.config, out);

            func.documentation.write(language_backend, out);

            if func.extern_decl {
                out.write("extern ");
            } else {
                if let Some(ref prefix) = prefix {
                    write!(out, "{}", prefix);
                    out.new_line();
                }
                if func.annotations.must_use(&language_backend.config) {
                    if let Some(ref anno) = language_backend.config.function.must_use {
                        write!(out, "{}", anno);
                        out.new_line();
                    }
                }
                if let Some(note) = func
                    .annotations
                    .deprecated_note(&language_backend.config, DeprecatedNoteKind::Function)
                {
                    write!(out, "{} ", note);
                }
            }
            cdecl::write_func(
                language_backend,
                out,
                func,
                Layout::Vertical,
                &language_backend.config,
            );
            if !func.extern_decl {
                if let Some(ref postfix) = postfix {
                    out.new_line();
                    write!(out, "{}", postfix);
                }
            }

            if let Some(ref swift_name_macro) = language_backend.config.function.swift_name_macro {
                if let Some(swift_name) = func.swift_name(&language_backend.config) {
                    write!(out, " {}({})", swift_name_macro, swift_name);
                }
            }

            out.write(";");

            condition.write_after(&language_backend.config, out);
        }

        match language_backend.config.function.args {
            Layout::Horizontal => write_1(self, language_backend, out),
            Layout::Vertical => write_2(self, language_backend, out),
            Layout::Auto => {
                if !out.try_write(
                    |out| write_1(self, language_backend, out),
                    language_backend.config.line_length,
                ) {
                    write_2(self, language_backend, out)
                }
            }
        }
    }
}

impl Source<CythonLanguageBackend> for Type {
    fn write<F: Write>(&self, language_backend: &CythonLanguageBackend, out: &mut SourceWriter<F>) {
        cdecl::write_type(language_backend, out, self, &language_backend.config);
    }
}

impl Source<CythonLanguageBackend> for Documentation {
    fn write<F: Write>(&self, language_backend: &CythonLanguageBackend, out: &mut SourceWriter<F>) {
        if self.doc_comment.is_empty() || !language_backend.config.documentation {
            return;
        }

        let end = match language_backend.config.documentation_length {
            DocumentationLength::Short => 1,
            DocumentationLength::Full => self.doc_comment.len(),
        };

        // Cython uses Python-style comments, so `documentation_style` is not relevant.
        for line in &self.doc_comment[..end] {
            write!(out, "#{}", line);
            out.new_line();
        }
    }
}

impl Source<CythonLanguageBackend> for Literal {
    fn write<F: Write>(&self, language_backend: &CythonLanguageBackend, out: &mut SourceWriter<F>) {
        match self {
            Literal::Expr(v) => match &**v {
                "true" => write!(out, "True"),
                "false" => write!(out, "False"),
                v => write!(out, "{}", v),
            },
            Literal::Path {
                ref associated_to,
                ref name,
            } => {
                if let Some((ref path, ref export_name)) = associated_to {
                    if let Some(known) = to_known_assoc_constant(path, name) {
                        return write!(out, "{}", known);
                    }
                    write!(out, "{}_", export_name)
                }
                write!(out, "{}", name)
            }
            Literal::FieldAccess {
                ref base,
                ref field,
            } => {
                write!(out, "(");
                base.write(language_backend, out);
                write!(out, ").{}", field);
            }
            Literal::PostfixUnaryOp { op, ref value } => {
                write!(out, "{}", op);
                value.write(language_backend, out);
            }
            Literal::BinOp {
                ref left,
                op,
                ref right,
            } => {
                write!(out, "(");
                left.write(language_backend, out);
                write!(out, " {} ", op);
                right.write(language_backend, out);
                write!(out, ")");
            }
            Literal::Cast { ref ty, ref value } => {
                out.write("<");
                ty.write(language_backend, out);
                out.write(">");
                value.write(language_backend, out);
            }
            Literal::Struct {
                export_name,
                fields,
                path,
            } => {
                write!(out, "<{}>", export_name);

                write!(out, "{{ ");
                let mut is_first_field = true;
                // In C++, same order as defined is required.
                let ordered_fields = out.bindings().struct_field_names(path);
                for ordered_key in ordered_fields.iter() {
                    if let Some(lit) = fields.get(ordered_key) {
                        if !is_first_field {
                            write!(out, ", ");
                        } else {
                            is_first_field = false;
                        }
                        lit.write(language_backend, out);
                    }
                }
                write!(out, " }}");
            }
        }
    }
}
