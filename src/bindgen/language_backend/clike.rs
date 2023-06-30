use crate::bindgen::ir::{
    to_known_assoc_constant, ConditionWrite, DeprecatedNoteKind, Documentation, Enum, EnumVariant,
    Field, Function, GenericParams, Item, Literal, OpaqueItem, ReprAlign, Static, Struct,
    ToCondition, Type, Typedef, Union,
};
use crate::bindgen::language_backend::{LanguageBackend, NamespaceOperation};
use crate::bindgen::rename::IdentifierType;
use crate::bindgen::writer::{ListType, Source, SourceWriter};
use crate::bindgen::{cdecl, Config, Language, Layout};
use crate::bindgen::{DocumentationLength, DocumentationStyle};
use std::io::Write;

pub struct CLikeLanguageBackend {
    config: Config,
}

impl CLikeLanguageBackend {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl LanguageBackend for CLikeLanguageBackend {
    fn write_headers<W: Write>(&self, out: &mut SourceWriter<W>) {
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

    fn open_close_namespaces<W: Write>(&self, op: NamespaceOperation, out: &mut SourceWriter<W>) {
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

        if op == NamespaceOperation::Close {
            namespaces.reverse();
        }

        if self.config.cpp_compatible_c() {
            out.new_line_if_not_start();
            out.write("#ifdef __cplusplus");
        }

        for namespace in namespaces {
            out.new_line();
            match op {
                NamespaceOperation::Open => write!(out, "namespace {} {{", namespace),
                NamespaceOperation::Close => write!(out, "}} // namespace {}", namespace),
            }
        }

        out.new_line();
        if self.config.cpp_compatible_c() {
            out.write("#endif // __cplusplus");
            out.new_line();
        }
    }

    fn write_footers<W: Write>(&self, out: &mut SourceWriter<W>) {
        if let Some(f) = self.config.include_guard() {
            out.new_line_if_not_start();
            if self.config.language == Language::C {
                write!(out, "#endif /* {} */", f);
            } else {
                write!(out, "#endif // {}", f);
            }
            out.new_line();
        }
    }
}

impl Source<CLikeLanguageBackend> for EnumVariant {
    fn write<F: Write>(&self, language_backend: &CLikeLanguageBackend, out: &mut SourceWriter<F>) {
        let condition = self.cfg.to_condition(&language_backend.config);

        condition.write_before(&language_backend.config, out);

        self.documentation.write(language_backend, out);
        write!(out, "{}", self.export_name);
        if let Some(discriminant) = &self.discriminant {
            out.write(" = ");

            discriminant.write(language_backend, out);
        }
        out.write(",");
        condition.write_after(&language_backend.config, out);
    }
}

impl Source<CLikeLanguageBackend> for Enum {
    fn write<F: Write>(&self, language_backend: &CLikeLanguageBackend, out: &mut SourceWriter<F>) {
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

        // If the enum has data, we need to emit a struct or union for the data
        // and enum for the tag. C++ supports nested type definitions, so we open
        // the struct or union here and define the tag enum inside it (*).
        if has_data && language_backend.config.language == Language::Cxx {
            self.open_struct_or_union(&language_backend.config, out, inline_tag_field);
        }

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

            // Open the struct or union for the data (**), gathering all the variants with data
            // together, unless it's C++, then we have already opened that struct/union at (*) and
            // are currently inside it.
            if language_backend.config.language != Language::Cxx {
                self.open_struct_or_union(&language_backend.config, out, inline_tag_field);
            }

            // Emit tag field that is separate from all variants.
            self.write_tag_field(
                &language_backend.config,
                out,
                size,
                inline_tag_field,
                tag_name,
            );
            out.new_line();

            // Open union of all variants with data, only in the non-inline tag scenario.
            if !inline_tag_field {
                out.write("union");
                out.open_brace();
            }

            // Emit fields for all variants with data.
            self.write_variant_fields(
                &language_backend.config,
                language_backend,
                out,
                inline_tag_field,
            );

            // Close union of all variants with data, only in the non-inline tag scenario.
            if !inline_tag_field {
                out.close_brace(true);
            }

            // Emit convenience methods for the struct or enum for the data.
            self.write_derived_functions_data(
                &language_backend.config,
                language_backend,
                out,
                tag_name,
            );

            // Emit the post_body section, if relevant.
            if let Some(body) = language_backend.config.export.post_body(&self.path) {
                out.new_line();
                out.write_raw_block(body);
            }

            // Close the struct or union opened either at (*) or at (**).
            if language_backend.config.language == Language::C
                && language_backend.config.style.generate_typedef()
            {
                out.close_brace(false);
                write!(out, " {};", self.export_name);
            } else {
                out.close_brace(true);
            }
        }

        condition.write_after(&language_backend.config, out);
    }
}

impl Source<CLikeLanguageBackend> for Struct {
    fn write<F: Write>(&self, language_backend: &CLikeLanguageBackend, out: &mut SourceWriter<F>) {
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

        // The following results in
        // C++ or C with Tag as style:
        //   struct Name {
        // C with Type only style:
        //   typedef struct {
        // C with Both as style:
        //   typedef struct Name {
        match language_backend.config.language {
            Language::C if language_backend.config.style.generate_typedef() => {
                out.write("typedef ")
            }
            Language::C | Language::Cxx => {}
            _ => unreachable!(),
        }

        out.write("struct");

        if let Some(align) = self.alignment {
            match align {
                ReprAlign::Packed => {
                    if let Some(ref anno) = language_backend.config.layout.packed {
                        write!(out, " {}", anno);
                    }
                }
                ReprAlign::Align(n) => {
                    if let Some(ref anno) = language_backend.config.layout.aligned_n {
                        write!(out, " {}({})", anno, n);
                    }
                }
            }
        }

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

        if language_backend.config.language != Language::C
            || language_backend.config.style.generate_tag()
        {
            write!(out, " {}", self.export_name());
        }

        out.open_brace();

        // Emit the pre_body section, if relevant
        if let Some(body) = language_backend.config.export.pre_body(&self.path) {
            out.write_raw_block(body);
            out.new_line();
        }

        out.write_vertical_source_list(language_backend, &self.fields, ListType::Cap(";"));

        if language_backend.config.language == Language::Cxx {
            let mut wrote_start_newline = false;

            if language_backend
                .config
                .structure
                .derive_constructor(&self.annotations)
                && !self.fields.is_empty()
            {
                if !wrote_start_newline {
                    wrote_start_newline = true;
                    out.new_line();
                }

                out.new_line();

                let arg_renamer = |name: &str| {
                    language_backend
                        .config
                        .function
                        .rename_args
                        .apply(name, IdentifierType::FunctionArg)
                        .into_owned()
                };
                write!(out, "{}(", self.export_name());
                let vec: Vec<_> = self
                    .fields
                    .iter()
                    .map(|field| {
                        Field::from_name_and_type(
                            // const-ref args to constructor
                            format!("const& {}", arg_renamer(&field.name)),
                            field.ty.clone(),
                        )
                    })
                    .collect();
                out.write_vertical_source_list(language_backend, &vec[..], ListType::Join(","));
                write!(out, ")");
                out.new_line();
                write!(out, "  : ");
                let vec: Vec<_> = self
                    .fields
                    .iter()
                    .map(|field| format!("{}({})", field.name, arg_renamer(&field.name)))
                    .collect();
                out.write_vertical_source_list(language_backend, &vec[..], ListType::Join(","));
                out.new_line();
                write!(out, "{{}}");
                out.new_line();
            }

            let other = language_backend
                .config
                .function
                .rename_args
                .apply("other", IdentifierType::FunctionArg);

            if self
                .annotations
                .bool("internal-derive-bitflags")
                .unwrap_or(false)
            {
                assert_eq!(self.fields.len(), 1);
                let bits = &self.fields[0].name;
                if !wrote_start_newline {
                    wrote_start_newline = true;
                    out.new_line();
                }
                let constexpr_prefix = if language_backend.config.constant.allow_constexpr {
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
                    self.export_name()
                );
                out.open_brace();
                write!(
                    out,
                    "return {} {{ static_cast<decltype({bits})>(~{bits}) }};",
                    self.export_name()
                );
                out.close_brace(false);
                self.emit_bitflags_binop(constexpr_prefix, '|', &other, out);
                self.emit_bitflags_binop(constexpr_prefix, '&', &other, out);
                self.emit_bitflags_binop(constexpr_prefix, '^', &other, out);
            }

            // Generate a serializer function that allows dumping this struct
            // to an std::ostream. It's defined as a friend function inside the
            // struct definition, and doesn't need the `inline` keyword even
            // though it's implemented right in the generated header file.
            if language_backend
                .config
                .structure
                .derive_ostream(&self.annotations)
            {
                if !wrote_start_newline {
                    wrote_start_newline = true;
                    out.new_line();
                }

                out.new_line();
                let stream = language_backend
                    .config
                    .function
                    .rename_args
                    .apply("stream", IdentifierType::FunctionArg);
                let instance = language_backend
                    .config
                    .function
                    .rename_args
                    .apply("instance", IdentifierType::FunctionArg);
                write!(
                    out,
                    "friend std::ostream& operator<<(std::ostream& {}, const {}& {})",
                    stream,
                    self.export_name(),
                    instance,
                );
                out.open_brace();
                write!(out, "return {} << \"{{ \"", stream);
                let vec: Vec<_> = self
                    .fields
                    .iter()
                    .map(|x| format!(" << \"{}=\" << {}.{}", x.name, instance, x.name))
                    .collect();
                out.write_vertical_source_list(
                    language_backend,
                    &vec[..],
                    ListType::Join(" << \", \""),
                );
                out.write(" << \" }\";");
                out.close_brace(false);
            }

            let skip_fields = self.has_tag_field as usize;

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

                    if let Some(Some(attrs)) =
                        self.annotations.atom(concat!($op_name, "-attributes"))
                    {
                        write!(out, "{} ", attrs);
                    }

                    write!(
                        out,
                        "bool operator{}(const {}& {}) const",
                        $op,
                        self.export_name(),
                        other
                    );
                    out.open_brace();
                    out.write("return ");
                    let vec: Vec<_> = self
                        .fields
                        .iter()
                        .skip(skip_fields)
                        .map(|field| format!("{} {} {}.{}", field.name, $op, other, field.name))
                        .collect();
                    out.write_vertical_source_list(
                        language_backend,
                        &vec[..],
                        ListType::Join(&format!(" {}", $conjuc)),
                    );
                    out.write(";");
                    out.close_brace(false);
                }};
            }

            if language_backend
                .config
                .structure
                .derive_eq(&self.annotations)
                && self.can_derive_eq()
            {
                emit_op!("eq", "==", "&&");
            }
            if language_backend
                .config
                .structure
                .derive_neq(&self.annotations)
                && self.can_derive_eq()
            {
                emit_op!("neq", "!=", "||");
            }
            if language_backend
                .config
                .structure
                .derive_lt(&self.annotations)
                && self.fields.len() == 1
                && self.fields[0].ty.can_cmp_order()
            {
                emit_op!("lt", "<", "&&");
            }
            if language_backend
                .config
                .structure
                .derive_lte(&self.annotations)
                && self.fields.len() == 1
                && self.fields[0].ty.can_cmp_order()
            {
                emit_op!("lte", "<=", "&&");
            }
            if language_backend
                .config
                .structure
                .derive_gt(&self.annotations)
                && self.fields.len() == 1
                && self.fields[0].ty.can_cmp_order()
            {
                emit_op!("gt", ">", "&&");
            }
            if language_backend
                .config
                .structure
                .derive_gte(&self.annotations)
                && self.fields.len() == 1
                && self.fields[0].ty.can_cmp_order()
            {
                emit_op!("gte", ">=", "&&");
            }
        }

        // Emit the post_body section, if relevant
        if let Some(body) = language_backend.config.export.post_body(&self.path) {
            out.new_line();
            out.write_raw_block(body);
        }

        if language_backend.config.language == Language::Cxx
            && language_backend
                .config
                .structure
                .associated_constants_in_body
            && language_backend.config.constant.allow_static_const
        {
            for constant in &self.associated_constants {
                out.new_line();
                constant.write_declaration(&language_backend.config, language_backend, out, self);
            }
        }

        if language_backend.config.language == Language::C
            && language_backend.config.style.generate_typedef()
        {
            out.close_brace(false);
            write!(out, " {};", self.export_name());
        } else {
            out.close_brace(true);
        }

        for constant in &self.associated_constants {
            out.new_line();
            constant.write(&language_backend.config, language_backend, out, Some(self));
        }

        condition.write_after(&language_backend.config, out);
    }
}

impl Source<CLikeLanguageBackend> for Union {
    fn write<F: Write>(&self, language_backend: &CLikeLanguageBackend, out: &mut SourceWriter<F>) {
        let condition = self.cfg.to_condition(&language_backend.config);
        condition.write_before(&language_backend.config, out);

        self.documentation.write(language_backend, out);

        self.generic_params.write(language_backend, out);

        // The following results in
        // C++ or C with Tag as style:
        //   union Name {
        // C with Type only style:
        //   typedef union {
        // C with Both as style:
        //   typedef union Name {
        match language_backend.config.language {
            Language::C if language_backend.config.style.generate_typedef() => {
                out.write("typedef ")
            }
            Language::C | Language::Cxx => {}
            _ => unreachable!(),
        }

        out.write("union");

        if let Some(align) = self.alignment {
            match align {
                ReprAlign::Packed => {
                    if let Some(ref anno) = language_backend.config.layout.packed {
                        write!(out, " {}", anno);
                    }
                }
                ReprAlign::Align(n) => {
                    if let Some(ref anno) = language_backend.config.layout.aligned_n {
                        write!(out, " {}({})", anno, n);
                    }
                }
            }
        }

        if language_backend.config.language != Language::C
            || language_backend.config.style.generate_tag()
        {
            write!(out, " {}", self.export_name);
        }

        out.open_brace();

        // Emit the pre_body section, if relevant
        if let Some(body) = language_backend.config.export.pre_body(&self.path) {
            out.write_raw_block(body);
            out.new_line();
        }

        out.write_vertical_source_list(language_backend, &self.fields, ListType::Cap(";"));

        // Emit the post_body section, if relevant
        if let Some(body) = language_backend.config.export.post_body(&self.path) {
            out.new_line();
            out.write_raw_block(body);
        }

        if language_backend.config.language == Language::C
            && language_backend.config.style.generate_typedef()
        {
            out.close_brace(false);
            write!(out, " {};", self.export_name);
        } else {
            out.close_brace(true);
        }

        condition.write_after(&language_backend.config, out);
    }
}

impl Source<CLikeLanguageBackend> for OpaqueItem {
    fn write<F: Write>(&self, language_backend: &CLikeLanguageBackend, out: &mut SourceWriter<F>) {
        let condition = self.cfg.to_condition(&language_backend.config);
        condition.write_before(&language_backend.config, out);

        self.documentation.write(language_backend, out);

        self.generic_params
            .write_with_default(language_backend, &language_backend.config, out);

        match language_backend.config.language {
            Language::C if language_backend.config.style.generate_typedef() => {
                write!(
                    out,
                    "typedef struct {} {};",
                    self.export_name(),
                    self.export_name()
                );
            }
            Language::C | Language::Cxx => {
                write!(out, "struct {};", self.export_name());
            }
            _ => unreachable!(),
        }

        condition.write_after(&language_backend.config, out);
    }
}

impl Source<CLikeLanguageBackend> for Field {
    fn write<F: Write>(&self, language_backend: &CLikeLanguageBackend, out: &mut SourceWriter<F>) {
        let condition = self.cfg.to_condition(&language_backend.config);
        condition.write_before(&language_backend.config, out);

        self.documentation.write(language_backend, out);
        cdecl::write_field(
            language_backend,
            out,
            &self.ty,
            &self.name,
            &language_backend.config,
        );

        if let Some(bitfield) = self.annotations.atom("bitfield") {
            write!(out, ": {}", bitfield.unwrap_or_default());
        }

        condition.write_after(&language_backend.config, out);
        // FIXME(#634): `write_vertical_source_list` should support
        // configuring list elements natively. For now we print a newline
        // here to avoid printing `#endif;` with semicolon.
        if condition.is_some() {
            out.new_line();
        }
    }
}

impl Source<CLikeLanguageBackend> for GenericParams {
    fn write<F: Write>(&self, language_backend: &CLikeLanguageBackend, out: &mut SourceWriter<F>) {
        self.write_internal(language_backend, &language_backend.config, out, false);
    }
}

impl Source<CLikeLanguageBackend> for Typedef {
    fn write<F: Write>(&self, language_backend: &CLikeLanguageBackend, out: &mut SourceWriter<F>) {
        let condition = self.cfg.to_condition(&language_backend.config);
        condition.write_before(&language_backend.config, out);

        self.documentation.write(language_backend, out);

        self.generic_params.write(language_backend, out);

        match language_backend.config.language {
            Language::Cxx => {
                write!(out, "using {} = ", self.export_name());
                self.aliased.write(language_backend, out);
            }
            Language::C => {
                write!(out, "{} ", language_backend.config.language.typedef());
                Field::from_name_and_type(self.export_name().to_owned(), self.aliased.clone())
                    .write(language_backend, out);
            }
            _ => unreachable!(),
        }

        out.write(";");

        condition.write_after(&language_backend.config, out);
    }
}

impl Source<CLikeLanguageBackend> for Static {
    fn write<F: Write>(&self, language_backend: &CLikeLanguageBackend, out: &mut SourceWriter<F>) {
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

impl Source<CLikeLanguageBackend> for Function {
    fn write<F: Write>(&self, language_backend: &CLikeLanguageBackend, out: &mut SourceWriter<F>) {
        fn write_1<W: Write>(
            func: &Function,
            language_backend: &CLikeLanguageBackend,
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
            language_backend: &CLikeLanguageBackend,
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
                    write!(out, "{}", note);
                    out.new_line();
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

impl Source<CLikeLanguageBackend> for Type {
    fn write<F: Write>(&self, language_backend: &CLikeLanguageBackend, out: &mut SourceWriter<F>) {
        cdecl::write_type(language_backend, out, self, &language_backend.config);
    }
}

impl Source<CLikeLanguageBackend> for Documentation {
    fn write<F: Write>(&self, language_backend: &CLikeLanguageBackend, out: &mut SourceWriter<F>) {
        if self.doc_comment.is_empty() || !language_backend.config.documentation {
            return;
        }

        let end = match language_backend.config.documentation_length {
            DocumentationLength::Short => 1,
            DocumentationLength::Full => self.doc_comment.len(),
        };

        let style = match language_backend.config.documentation_style {
            DocumentationStyle::Auto if language_backend.config.language == Language::C => {
                DocumentationStyle::Doxy
            }
            DocumentationStyle::Auto if language_backend.config.language == Language::Cxx => {
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

        for line in &self.doc_comment[..end] {
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
}

impl Source<CLikeLanguageBackend> for Literal {
    fn write<F: Write>(&self, language_backend: &CLikeLanguageBackend, out: &mut SourceWriter<F>) {
        match self {
            Literal::Expr(v) => match (&**v, language_backend.config.language) {
                ("true", Language::Cython) => write!(out, "True"),
                ("false", Language::Cython) => write!(out, "False"),
                (v, _) => write!(out, "{}", v),
            },
            Literal::Path {
                ref associated_to,
                ref name,
            } => {
                if let Some((ref path, ref export_name)) = associated_to {
                    if let Some(known) = to_known_assoc_constant(path, name) {
                        return write!(out, "{}", known);
                    }
                    let path_separator = match language_backend.config.language {
                        Language::Cython | Language::C => "_",
                        Language::Cxx => {
                            if language_backend
                                .config
                                .structure
                                .associated_constants_in_body
                            {
                                "::"
                            } else {
                                "_"
                            }
                        }
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
                out.write("(");
                ty.write(language_backend, out);
                out.write(")");
                value.write(language_backend, out);
            }
            Literal::Struct {
                export_name,
                fields,
                path,
            } => {
                match language_backend.config.language {
                    Language::C => write!(out, "({})", export_name),
                    Language::Cxx => write!(out, "{}", export_name),
                    _ => unreachable!(),
                }

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
                        match language_backend.config.language {
                            Language::Cxx => write!(out, "/* .{} = */ ", ordered_key),
                            Language::C => write!(out, ".{} = ", ordered_key),
                            _ => unreachable!(),
                        }
                        lit.write(language_backend, out);
                    }
                }
                write!(out, " }}");
            }
        }
    }
}
