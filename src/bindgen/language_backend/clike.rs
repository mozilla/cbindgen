use crate::bindgen::ir::{
    to_known_assoc_constant, ConditionWrite, DeprecatedNoteKind, Documentation, Enum, EnumVariant,
    Field, Function, GenericParams, Item, Literal, OpaqueItem, ReprAlign, Static, Struct,
    ToCondition, Type, Typedef, Union,
};
use crate::bindgen::language_backend::LanguageBackend;
use crate::bindgen::rename::IdentifierType;
use crate::bindgen::writer::{ListType, SourceWriter};
use crate::bindgen::{cdecl, Bindings, Config, Language, Layout};
use crate::bindgen::{DocumentationLength, DocumentationStyle};
use std::io::Write;

pub struct CLikeLanguageBackend<'a> {
    config: &'a Config,
}

impl<'a> CLikeLanguageBackend<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    fn write_enum_variant<W: Write>(&mut self, out: &mut SourceWriter<W>, u: &EnumVariant) {
        let condition = u.cfg.to_condition(self.config);

        condition.write_before(self.config, out);

        self.write_documentation(out, &u.documentation);
        write!(out, "{}", u.export_name);
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
                write!(out, "}} // namespace {}", namespace)
            }
        }

        out.new_line();
        if self.config.cpp_compatible_c() {
            out.write("#endif // __cplusplus");
            out.new_line();
        }
    }
}

impl LanguageBackend for CLikeLanguageBackend<'_> {
    fn write_headers<W: Write>(&mut self, out: &mut SourceWriter<W>) {
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
                write!(out, "#endif /* {} */", f);
            } else {
                write!(out, "#endif // {}", f);
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
            if self.config.language == Language::C && self.config.style.generate_typedef() {
                out.close_brace(false);
                write!(out, " {};", e.export_name);
            } else {
                out.close_brace(true);
            }
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

        // The following results in
        // C++ or C with Tag as style:
        //   struct Name {
        // C with Type only style:
        //   typedef struct {
        // C with Both as style:
        //   typedef struct Name {
        match self.config.language {
            Language::C if self.config.style.generate_typedef() => out.write("typedef "),
            Language::C | Language::Cxx => {}
            _ => unreachable!(),
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
                out.write_vertical_source_list(
                    self,
                    &vec[..],
                    ListType::Join(","),
                    Self::write_field,
                );
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

                    if let Some(Some(attrs)) = s.annotations.atom(concat!($op_name, "-attributes"))
                    {
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

        if self.config.language == Language::C && self.config.style.generate_typedef() {
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
        match self.config.language {
            Language::C if self.config.style.generate_typedef() => out.write("typedef "),
            Language::C | Language::Cxx => {}
            _ => unreachable!(),
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

        if self.config.language == Language::C && self.config.style.generate_typedef() {
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

        match self.config.language {
            Language::C if self.config.style.generate_typedef() => {
                write!(
                    out,
                    "typedef struct {} {};",
                    o.export_name(),
                    o.export_name()
                );
            }
            Language::C | Language::Cxx => {
                write!(out, "struct {};", o.export_name());
            }
            _ => unreachable!(),
        }

        condition.write_after(self.config, out);
    }

    fn write_type_def<W: Write>(&mut self, out: &mut SourceWriter<W>, t: &Typedef) {
        let condition = t.cfg.to_condition(self.config);
        condition.write_before(self.config, out);

        self.write_documentation(out, &t.documentation);

        self.write_generic_param(out, &t.generic_params);

        match self.config.language {
            Language::Cxx => {
                write!(out, "using {} = ", t.export_name());
                self.write_type(out, &t.aliased);
            }
            Language::C => {
                write!(out, "{} ", self.config.language.typedef());
                self.write_field(
                    out,
                    &Field::from_name_and_type(t.export_name().to_owned(), t.aliased.clone()),
                )
            }
            _ => unreachable!(),
        }

        out.write(";");

        condition.write_after(self.config, out);
    }

    fn write_static<W: Write>(&mut self, out: &mut SourceWriter<W>, s: &Static) {
        out.write("extern ");
        if let Type::Ptr { is_const: true, .. } = s.ty {
        } else if !s.mutable {
            out.write("const ");
        }
        cdecl::write_field(self, out, &s.ty, &s.export_name, self.config);
        out.write(";");
    }

    fn write_function<W: Write>(&mut self, out: &mut SourceWriter<W>, f: &Function) {
        fn write_1<W: Write>(
            func: &Function,
            language_backend: &mut CLikeLanguageBackend,
            out: &mut SourceWriter<W>,
        ) {
            let prefix = language_backend.config.function.prefix(&func.annotations);
            let postfix = language_backend.config.function.postfix(&func.annotations);

            let condition = func.cfg.to_condition(language_backend.config);
            condition.write_before(language_backend.config, out);

            language_backend.write_documentation(out, &func.documentation);

            if func.extern_decl {
                out.write("extern ");
            } else {
                if let Some(ref prefix) = prefix {
                    write!(out, "{} ", prefix);
                }
                if func.annotations.must_use(language_backend.config) {
                    if let Some(ref anno) = language_backend.config.function.must_use {
                        write!(out, "{} ", anno);
                    }
                }
                if let Some(note) = func
                    .annotations
                    .deprecated_note(language_backend.config, DeprecatedNoteKind::Function)
                {
                    write!(out, "{} ", note);
                }
            }
            cdecl::write_func(
                language_backend,
                out,
                func,
                Layout::Horizontal,
                language_backend.config,
            );

            if !func.extern_decl {
                if let Some(ref postfix) = postfix {
                    write!(out, " {}", postfix);
                }
            }

            if let Some(ref swift_name_macro) = language_backend.config.function.swift_name_macro {
                if let Some(swift_name) = func.swift_name(language_backend.config) {
                    write!(out, " {}({})", swift_name_macro, swift_name);
                }
            }

            out.write(";");

            condition.write_after(language_backend.config, out);
        }

        fn write_2<W: Write>(
            func: &Function,
            language_backend: &mut CLikeLanguageBackend,
            out: &mut SourceWriter<W>,
        ) {
            let prefix = language_backend.config.function.prefix(&func.annotations);
            let postfix = language_backend.config.function.postfix(&func.annotations);

            let condition = func.cfg.to_condition(language_backend.config);

            condition.write_before(language_backend.config, out);

            language_backend.write_documentation(out, &func.documentation);

            if func.extern_decl {
                out.write("extern ");
            } else {
                if let Some(ref prefix) = prefix {
                    write!(out, "{}", prefix);
                    out.new_line();
                }
                if func.annotations.must_use(language_backend.config) {
                    if let Some(ref anno) = language_backend.config.function.must_use {
                        write!(out, "{}", anno);
                        out.new_line();
                    }
                }
                if let Some(note) = func
                    .annotations
                    .deprecated_note(language_backend.config, DeprecatedNoteKind::Function)
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
                language_backend.config,
            );
            if !func.extern_decl {
                if let Some(ref postfix) = postfix {
                    out.new_line();
                    write!(out, "{}", postfix);
                }
            }

            if let Some(ref swift_name_macro) = language_backend.config.function.swift_name_macro {
                if let Some(swift_name) = func.swift_name(language_backend.config) {
                    write!(out, " {}({})", swift_name_macro, swift_name);
                }
            }

            out.write(";");

            condition.write_after(language_backend.config, out);
        }

        match self.config.function.args {
            Layout::Horizontal => write_1(f, self, out),
            Layout::Vertical => write_2(f, self, out),
            Layout::Auto => {
                let max_line_length = self.config.line_length;
                if !out.try_write(|out| write_1(f, self, out), max_line_length) {
                    write_2(f, self, out)
                }
            }
        }
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
            Literal::Expr(v) => match (&**v, self.config.language) {
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
                    let path_separator = match self.config.language {
                        Language::Cython | Language::C => "_",
                        Language::Cxx => {
                            if self.config.structure.associated_constants_in_body {
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
                match self.config.language {
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
                        match self.config.language {
                            Language::Cxx => write!(out, "/* .{} = */ ", ordered_key),
                            Language::C => write!(out, ".{} = ", ordered_key),
                            _ => unreachable!(),
                        }
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
        // Override default method to close various blocs containing both globals and functions
        // these blocks are opened in [`write_globals`] that is also overridden
        if !b.functions.is_empty() || !b.globals.is_empty() {
            self.write_functions_default(out, b);

            if b.config.cpp_compatible_c() {
                out.new_line();
                out.write("#ifdef __cplusplus");
            }

            if b.config.language == Language::Cxx || b.config.cpp_compatible_c() {
                out.new_line();
                out.write("} // extern \"C\"");
                out.new_line();
            }

            if b.config.cpp_compatible_c() {
                out.write("#endif // __cplusplus");
                out.new_line();
            }
        }
    }
}
