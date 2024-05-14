use crate::bindgen::ir::{
    Constant, Documentation, Enum, Field, Function, IntKind, Item, Literal, OpaqueItem,
    PrimitiveType, Static, Struct, Type, Typedef, Union,
};
use crate::bindgen::language_backend::LanguageBackend;
use crate::bindgen::writer::ListType::Join;
use crate::bindgen::writer::SourceWriter;
use crate::bindgen::{Config, Layout};
use std::fmt::Debug;
use std::io::Write;

pub struct JavaJnaLanguageBackend<'a> {
    config: &'a Config,
    binding_lib_crate_name: String,
    seen_size: bool,
    seen_size_t: bool,
    seen_boolean: bool,
}

impl<'a> JavaJnaLanguageBackend<'a> {
    pub fn new(config: &'a Config, binding_lib_crate_name: String) -> Self {
        Self {
            config,
            binding_lib_crate_name,
            seen_size: false,
            seen_size_t: false,
            seen_boolean: false,
        }
    }
}

impl LanguageBackend for JavaJnaLanguageBackend<'_> {
    fn open_namespaces<W: Write>(&mut self, out: &mut SourceWriter<W>) {
        out.new_line_if_not_start();
        let name = &self
            .config
            .java_jna
            .interface_name
            .as_deref()
            .unwrap_or("Bindings");

        write!(out, "enum {}Singleton", name);
        out.open_brace();
        out.write("INSTANCE;");
        out.new_line();

        write!(
            out,
            "final {} lib = Native.load(\"{}\", {}.class);",
            name, self.binding_lib_crate_name, name
        );
        out.close_brace(false);
        out.new_line();
        out.new_line();

        write!(out, "interface {} extends Library", name);
        out.open_brace();

        write!(out, "{} INSTANCE = {}Singleton.INSTANCE.lib;", name, name);
        out.new_line();

        if let Some(extra) = &self.config.java_jna.extra_defs {
            write!(out, "{extra}");
            out.new_line();
        }
    }

    fn close_namespaces<W: Write>(&mut self, out: &mut SourceWriter<W>) {
        if self.seen_size {
            self.write_integer_type(
                out,
                &JnaIntegerType {
                    documentation: &Documentation::none(),
                    name: "_Size",
                    underlying_jna_integer_type: UnderlyingJnaIntegerType::Size,
                    signed: false,
                    deprecated: None,
                },
                |_, _| {},
            )
        }
        if self.seen_size_t {
            self.write_integer_type(
                out,
                &JnaIntegerType {
                    documentation: &Documentation::none(),
                    name: "_SizeT",
                    underlying_jna_integer_type: UnderlyingJnaIntegerType::SizeT,
                    signed: false,
                    deprecated: None,
                },
                |_, _| {},
            )
        }
        if self.seen_boolean {
            self.write_integer_type(
                out,
                &JnaIntegerType {
                    documentation: &Documentation::none(),
                    name: "_Boolean",
                    underlying_jna_integer_type: UnderlyingJnaIntegerType::Byte,
                    signed: false,
                    deprecated: None,
                },
                |_, out| {
                    out.new_line();
                    write!(out, "public static final _Boolean FALSE = new _Boolean(0);");
                    out.new_line();
                    write!(out, "public static final _Boolean TRUE = new _Boolean(1);");
                },
            )
        }
        out.close_brace(false);
    }

    fn write_headers<W: Write>(&self, out: &mut SourceWriter<W>, package_version: &str) {
        if let Some(ref header) = self.config.header {
            out.new_line_if_not_start();
            write!(out, "{header}");
            out.new_line();
        }

        if self.config.package_version {
            out.new_line_if_not_start();
            write!(out, "/* Package version: {} */", package_version);
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
        if let Some(ref autogen_warning) = self.config.autogen_warning {
            out.new_line_if_not_start();
            write!(out, "{autogen_warning}");
            out.new_line();
        }

        if let Some(ref package) = self.config.java_jna.package {
            out.new_line_if_not_start();
            write!(out, "package {package};");
            out.new_line();
            out.new_line();
        }

        if !self.config.no_includes {
            out.write("import com.sun.jna.*;");
            out.new_line();
            out.write("import com.sun.jna.ptr.*;");
            out.new_line();
        }
    }

    fn write_footers<W: Write>(&mut self, _: &mut SourceWriter<W>) {}

    fn write_enum<W: Write>(&mut self, out: &mut SourceWriter<W>, e: &Enum) {
        self.write_integer_type(
            out,
            &JnaIntegerType {
                documentation: &e.documentation,
                name: &e.export_name,
                /* enum are most of the time the same size as ints */
                underlying_jna_integer_type: UnderlyingJnaIntegerType::Int,
                signed: false,
                deprecated: e.annotations.deprecated.as_deref(),
            },
            |lb, out| {
                let mut current_discriminant = 0;
                for variant in &e.variants {
                    current_discriminant = variant
                        .discriminant
                        .clone()
                        .and_then(|it| match it {
                            Literal::Expr(e) => e.parse::<i32>().ok(),
                            _ => None,
                        })
                        .unwrap_or(current_discriminant + 1);
                    lb.write_documentation(out, &variant.documentation);
                    write!(
                        out,
                        "public static final {} {} = new {}({});",
                        e.export_name, variant.export_name, e.export_name, current_discriminant
                    );
                    out.new_line();
                }
            },
        );
    }

    fn write_struct<W: Write>(&mut self, out: &mut SourceWriter<W>, s: &Struct) {
        let constants: Vec<(&Constant, &Struct)> =
            s.associated_constants.iter().map(|it| (it, s)).collect();
        if s.is_transparent {
            let field = s.fields.first();
            match field {
                Some(Field {
                    ty: Type::Primitive(PrimitiveType::Integer { kind, signed, .. }),
                    ..
                }) => {
                    self.write_integer_type(
                        out,
                        &JnaIntegerType {
                            documentation: &s.documentation,
                            name: &s.export_name,
                            underlying_jna_integer_type: UnderlyingJnaIntegerType::from_kind(kind),
                            signed: *signed,
                            deprecated: s.annotations.deprecated.as_deref(),
                        },
                        |lb, out| {
                            for (constant, assoc_struct) in constants {
                                constant.write(lb.config, lb, out, Some(assoc_struct));
                            }
                        },
                    );
                }
                Some(Field {
                    ty: Type::Path(path),
                    ..
                }) => {
                    self.write_jna_struct(
                        out,
                        &JnaStruct {
                            documentation: &s.documentation,
                            constants: &constants,
                            fields: &vec![],
                            name: &s.export_name,
                            superclass: path.export_name(),
                            interface: "Structure.ByValue",
                            deprecated: s.annotations.deprecated.as_deref(),
                        },
                    );
                    self.write_jna_struct(
                        out,
                        &JnaStruct {
                            documentation: &s.documentation,
                            constants: &constants,
                            fields: &vec![],
                            name: &format!("{}ByReference", s.export_name()),
                            superclass: path.export_name(),
                            interface: "Structure.ByReference",
                            deprecated: s.annotations.deprecated.as_deref(),
                        },
                    );
                }
                Some(Field {
                    ty: Type::Array(_, _),
                    ..
                }) => self.write_pointer_type(
                    out,
                    &s.documentation,
                    s.annotations.deprecated.as_deref(),
                    s.export_name(),
                ),
                _ => not_implemented(s, out),
            }
        } else {
            self.write_jna_struct(
                out,
                &JnaStruct {
                    documentation: &s.documentation,
                    constants: &constants,
                    fields: &s.fields,
                    name: &s.export_name,
                    superclass: "Structure",
                    interface: "Structure.ByValue",
                    deprecated: s.annotations.deprecated.as_deref(),
                },
            );
            self.write_jna_struct(
                out,
                &JnaStruct {
                    documentation: &s.documentation,
                    constants: &constants,
                    fields: &s.fields,
                    name: &format!("{}ByReference", s.export_name),
                    superclass: "Structure",
                    interface: "Structure.ByReference",
                    deprecated: s.annotations.deprecated.as_deref(),
                },
            );
        }
    }

    fn write_union<W: Write>(&mut self, out: &mut SourceWriter<W>, u: &Union) {
        self.write_jna_struct(
            out,
            &JnaStruct {
                documentation: &u.documentation,
                constants: &vec![],
                fields: &u.fields,
                name: &u.export_name,
                superclass: "Union",
                interface: "Structure.ByValue",
                deprecated: u.annotations.deprecated.as_deref(),
            },
        );
        self.write_jna_struct(
            out,
            &JnaStruct {
                documentation: &u.documentation,
                constants: &vec![],
                fields: &u.fields,
                name: &format!("{}ByReference", &u.export_name),
                superclass: "Union",
                interface: "Structure.ByReference",
                deprecated: u.annotations.deprecated.as_deref(),
            },
        );
    }

    fn write_opaque_item<W: Write>(&mut self, out: &mut SourceWriter<W>, o: &OpaqueItem) {
        self.write_pointer_type(
            out,
            &o.documentation,
            o.annotations.deprecated.as_deref(),
            &o.export_name,
        );
    }
    fn write_type_def<W: Write>(&mut self, out: &mut SourceWriter<W>, t: &Typedef) {
        match &t.aliased {
            Type::FuncPtr { ret, args, .. } => {
                write!(out, "interface {} extends Callback", t.export_name);
                out.open_brace();
                self.write_type(out, ret);
                out.write(" invoke(");
                self.write_indexed_function_args(
                    out,
                    &args
                        .iter()
                        .enumerate()
                        .map(|(index, (name, ty))| IndexedFunctionArg { name, index, ty })
                        .collect::<Vec<_>>(),
                );
                out.write(");");
                out.close_brace(false)
            }
            Type::Path(path) => {
                self.write_documentation(out, &t.documentation);
                write!(
                    out,
                    "class {} extends {}",
                    t.export_name,
                    path.export_name()
                );
                out.open_brace();
                write!(out, "public {}()", t.export_name);
                out.open_brace();
                out.write("super();");
                out.close_brace(false);
                out.new_line();
                write!(out, "public {}(Pointer p)", t.export_name);
                out.open_brace();
                out.write("super(p);");
                out.close_brace(false);
                out.close_brace(false);
                out.new_line();
                out.new_line();
                self.write_documentation(out, &t.documentation);
                write!(
                    out,
                    "class {}ByReference extends {}ByReference",
                    t.export_name,
                    path.export_name()
                );
                out.open_brace();
                write!(out, "public {}ByReference()", t.export_name);
                out.open_brace();
                out.write("super();");
                out.close_brace(false);
                out.new_line();
                write!(out, "public {}ByReference(Pointer p)", t.export_name);
                out.open_brace();
                out.write("super(p);");
                out.close_brace(false);
                out.close_brace(false);
            }
            Type::Primitive(primitive) => match primitive {
                PrimitiveType::Integer { kind, signed, .. } => {
                    let jna_type = UnderlyingJnaIntegerType::from_kind(kind);
                    self.write_integer_type(
                        out,
                        &JnaIntegerType {
                            documentation: &t.documentation,
                            name: &t.export_name,
                            underlying_jna_integer_type: jna_type,
                            signed: *signed,
                            deprecated: t.annotations.deprecated.as_deref(),
                        },
                        |_, _| {},
                    )
                }
                _ => not_implemented(&t, out),
            },
            Type::Ptr { .. } => self.write_pointer_type(
                out,
                &t.documentation,
                t.annotations.deprecated.as_deref(),
                &t.export_name,
            ),
            Type::Array(_, _) => self.write_pointer_type(
                out,
                &t.documentation,
                t.annotations.deprecated.as_deref(),
                &t.export_name,
            ),
        }
    }

    fn write_static<W: Write>(&mut self, out: &mut SourceWriter<W>, s: &Static) {
        not_implemented(s, out)
    }

    fn write_function<W: Write>(
        &mut self,
        _config: &Config,
        out: &mut SourceWriter<W>,
        f: &Function,
    ) {
        self.write_documentation(out, &f.documentation);
        self.write_deprecated(out, f.annotations.deprecated.as_deref());
        self.write_type(out, &f.ret);
        write!(out, " {}(", f.path.name());

        self.write_indexed_function_args(
            out,
            &f.args
                .iter()
                .enumerate()
                .map(|(index, arg)| IndexedFunctionArg {
                    name: &arg.name,
                    ty: &arg.ty,
                    index,
                })
                .collect::<Vec<_>>(),
        );

        out.write(");");
    }

    fn write_type<W: Write>(&mut self, out: &mut SourceWriter<W>, t: &Type) {
        match t {
            Type::Ptr { ty, .. } => match &**ty {
                Type::Ptr { .. } => out.write("PointerByReference"),
                Type::Path(path) => {
                    write!(out, "{}ByReference", path.export_name())
                }
                Type::Primitive(primitive) => {
                    let typ = match primitive {
                        PrimitiveType::Void => "Pointer",
                        PrimitiveType::Bool => {
                            self.seen_boolean = true;
                            // let's not use java's boolean as it has a 4 byte size instead of 1 byte
                            "_Boolean"
                        }
                        PrimitiveType::Char => "ByteByReference",
                        PrimitiveType::SChar => "ByteByReference",
                        PrimitiveType::UChar => "ByteByReference",
                        PrimitiveType::Char32 => "Pointer",
                        PrimitiveType::Float => "FloatByReference",
                        PrimitiveType::Double => "DoubleByReference",
                        PrimitiveType::VaList => "PointerByReference",
                        PrimitiveType::PtrDiffT => "PointerByReference",
                        PrimitiveType::Integer { kind, .. } => match kind {
                            IntKind::Short => "ShortByReference",
                            IntKind::Int => "IntByReference",
                            IntKind::Long => "NativeLongByReference",
                            IntKind::LongLong => "LongByReference",
                            IntKind::SizeT => {
                                self.seen_size_t = true;
                                "_SizeTByReference"
                            }
                            IntKind::Size => {
                                self.seen_size = true;
                                "_SizeByReference"
                            }
                            IntKind::B8 => "ByteByReference",
                            IntKind::B16 => "ShortByReference",
                            IntKind::B32 => "IntByReference",
                            IntKind::B64 => "LongByReference",
                        },
                    };
                    write!(out, "{typ}")
                }
                Type::Array(_, _) => out.write("Pointer"),
                Type::FuncPtr { .. } => out.write("CallbackReference"),
            },

            Type::Path(path) => {
                write!(out, "{}", path.export_name())
            }
            Type::Primitive(primitive) => {
                // https://github.com/java-native-access/jna/blob/master/www/Mappings.md
                let typ = match primitive {
                    PrimitiveType::Void => "void",
                    PrimitiveType::Bool => {
                        self.seen_boolean = true;
                        // let's not use java's boolean as it has a 4 byte size instead of 1 byte
                        "_Boolean"
                    }
                    PrimitiveType::Char => "byte",
                    PrimitiveType::SChar => "byte",
                    PrimitiveType::UChar => "byte",
                    PrimitiveType::Char32 => "char",
                    PrimitiveType::Float => "float",
                    PrimitiveType::Double => "double",
                    PrimitiveType::VaList => "Pointer",
                    PrimitiveType::PtrDiffT => "Pointer",
                    PrimitiveType::Integer { kind, .. } => match kind {
                        IntKind::Short => "short",
                        IntKind::Int => "int",
                        IntKind::Long => "NativeLong",
                        IntKind::LongLong => "long",
                        IntKind::SizeT => {
                            self.seen_size_t = true;
                            "_SizeT"
                        }
                        IntKind::Size => {
                            self.seen_size = true;
                            "_Size"
                        }
                        IntKind::B8 => "byte",
                        IntKind::B16 => "short",
                        IntKind::B32 => "int",
                        IntKind::B64 => "long",
                    },
                };
                write!(out, "{typ}")
            }
            Type::Array(ty, _len) => {
                self.write_type(out, ty);
                out.write("[]");
            }
            Type::FuncPtr { .. } => out.write("Callback"),
        }
    }

    fn write_documentation<W: Write>(&mut self, out: &mut SourceWriter<W>, d: &Documentation) {
        if !d.doc_comment.is_empty() {
            out.new_line_if_not_start();
            out.write("/**");
            for line in &d.doc_comment {
                out.new_line();
                write!(out, " *{line}")
            }
            out.new_line();
            out.write(" */");
            out.new_line();
        }
    }

    fn write_literal<W: Write>(&mut self, out: &mut SourceWriter<W>, l: &Literal) {
        match l {
            Literal::Expr(expr) => {
                write!(out, "{expr}")
            }
            Literal::Struct { export_name, .. } => {
                // There is an hashmap in there that doesn't have stable debug output
                not_implemented(&format!("Struct Literal {export_name}"), out)
            }
            _ => not_implemented(l, out),
        }
    }
}

enum UnderlyingJnaIntegerType {
    Byte,
    Short,
    Int,
    NativeLong,
    Long,
    SizeT,
    Size,
}

impl UnderlyingJnaIntegerType {
    pub fn size(&self) -> &str {
        match self {
            UnderlyingJnaIntegerType::Byte => "1",
            UnderlyingJnaIntegerType::Short => "2",
            UnderlyingJnaIntegerType::Int => "4",
            UnderlyingJnaIntegerType::NativeLong => "Native.LONG_SIZE",
            UnderlyingJnaIntegerType::Long => "8",
            UnderlyingJnaIntegerType::SizeT => "Native.SIZE_T_SIZE",
            UnderlyingJnaIntegerType::Size => "Native.POINTER_SIZE",
        }
    }

    pub fn set_method(&self) -> &str {
        match self {
            UnderlyingJnaIntegerType::Byte => "p.setByte(0, (byte)value.intValue());",
            UnderlyingJnaIntegerType::Short => "p.setShort(0, (short)value.intValue());",
            UnderlyingJnaIntegerType::Int => "p.setInt(0, value.intValue());",
            UnderlyingJnaIntegerType::NativeLong => {
                "p.setNativeLong(0, new NativeLong(value.longValue()));"
            }
            UnderlyingJnaIntegerType::SizeT => "if (Native.SIZE_T_SIZE == 8) { p.setLong(0, value.longValue()); } else { p.setInt(0, value.intValue()); }",
            UnderlyingJnaIntegerType::Size => "if (Native.POINTER_SIZE == 8) { p.setLong(0, value.longValue()); } else { p.setInt(0, value.intValue()); }",
            UnderlyingJnaIntegerType::Long => "p.setLong(0, value.longValue());",
        }
    }

    pub fn get_method(&self) -> &str {
        match self {
            UnderlyingJnaIntegerType::Byte => "p.getByte(0)",
            UnderlyingJnaIntegerType::Short => "p.getShort(0)",
            UnderlyingJnaIntegerType::Int => "p.getInt(0)",
            UnderlyingJnaIntegerType::NativeLong => "p.getNativeLong(0).longValue()",
            UnderlyingJnaIntegerType::SizeT => {
                "Native.SIZE_T_SIZE == 8 ? p.getLong(0) : p.getInt(0)"
            }
            UnderlyingJnaIntegerType::Size => {
                "Native.POINTER_SIZE == 8 ? p.getLong(0) : p.getInt(0)"
            }
            UnderlyingJnaIntegerType::Long => "p.getLong(0)",
        }
    }

    pub fn from_kind(kind: &IntKind) -> Self {
        match kind {
            IntKind::Short => UnderlyingJnaIntegerType::Short,
            IntKind::Int => UnderlyingJnaIntegerType::Int,
            IntKind::Long => UnderlyingJnaIntegerType::NativeLong,
            IntKind::LongLong => UnderlyingJnaIntegerType::Long,
            IntKind::SizeT => UnderlyingJnaIntegerType::SizeT,
            IntKind::Size => UnderlyingJnaIntegerType::Size,
            IntKind::B8 => UnderlyingJnaIntegerType::Byte,
            IntKind::B16 => UnderlyingJnaIntegerType::Short,
            IntKind::B32 => UnderlyingJnaIntegerType::Int,
            IntKind::B64 => UnderlyingJnaIntegerType::Long,
        }
    }
}

struct JnaStruct<'a> {
    documentation: &'a Documentation,
    constants: &'a Vec<(&'a Constant, &'a Struct)>,
    fields: &'a Vec<Field>,
    name: &'a str,
    superclass: &'a str,
    interface: &'a str,
    deprecated: Option<&'a str>,
}

struct JnaIntegerType<'a> {
    documentation: &'a Documentation,
    name: &'a str,
    underlying_jna_integer_type: UnderlyingJnaIntegerType,
    signed: bool,
    deprecated: Option<&'a str>,
}

struct IndexedFunctionArg<'a> {
    ty: &'a Type,
    name: &'a Option<String>,
    index: usize,
}

impl JavaJnaLanguageBackend<'_> {
    fn write_deprecated<F: Write>(&mut self, out: &mut SourceWriter<F>, deprecated: Option<&str>) {
        if let Some(deprecated) = deprecated {
            if !deprecated.is_empty() {
                out.write("/**");
                out.new_line();
                write!(out, " * @deprecated {}", deprecated);
                out.new_line();
                out.write(" */");
                out.new_line();
            }
            out.write("@Deprecated");
            out.new_line()
        }
    }

    fn write_jna_struct<F: Write>(&mut self, out: &mut SourceWriter<F>, s: &JnaStruct) {
        out.new_line();
        self.write_documentation(out, s.documentation);
        self.write_deprecated(out, s.deprecated);
        let field_names = s
            .fields
            .iter()
            .map(|it| format!("\"{}\"", it.name))
            .collect::<Vec<_>>();

        if !field_names.is_empty() {
            out.write("@Structure.FieldOrder({");
            let max_line_length = self.config.line_length;
            if !out.try_write(
                |out| {
                    out.write_horizontal_source_list(self, &field_names, Join(", "), |_, out, s| {
                        write!(out, "{}", s)
                    })
                },
                max_line_length,
            ) {
                out.write_vertical_source_list(self, &field_names, Join(","), |_, out, s| {
                    write!(out, "{}", s)
                })
            }
            out.write("})");
            out.new_line();
        }
        write!(
            out,
            "class {} extends {} implements {}",
            s.name, s.superclass, s.interface
        );
        out.open_brace();

        for (constant, assoc_struct) in s.constants {
            constant.write(self.config, self, out, Some(assoc_struct));
        }

        write!(out, "public {}()", s.name);
        out.open_brace();
        out.write("super();");
        out.close_brace(false);
        out.new_line();
        out.new_line();

        write!(out, "public {}(Pointer p)", s.name);
        out.open_brace();
        out.write("super(p);");
        out.close_brace(false);
        out.new_line();
        out.new_line();

        for field in s.fields {
            self.write_documentation(out, &field.documentation);
            out.write("public ");
            self.write_type(out, &field.ty);
            write!(out, " {};", field.name);

            out.new_line()
        }

        out.close_brace(false);
        out.new_line();
    }

    fn write_indexed_function_arg<W: Write>(
        &mut self,
        out: &mut SourceWriter<W>,
        a: &IndexedFunctionArg,
    ) {
        self.write_type(out, a.ty);
        write!(
            out,
            " {}",
            a.name
                .clone()
                .and_then(|it| if it == "_" { None } else { Some(it) })
                .unwrap_or(format!("arg{}", a.index))
        );
    }

    fn write_indexed_function_args<W: Write>(
        &mut self,
        out: &mut SourceWriter<W>,
        a: &[IndexedFunctionArg],
    ) {
        match self.config.function.args {
            Layout::Horizontal => out.write_horizontal_source_list(
                self,
                a,
                Join(", "),
                Self::write_indexed_function_arg,
            ),
            Layout::Vertical => out.write_vertical_source_list(
                self,
                a,
                Join(", "),
                Self::write_indexed_function_arg,
            ),
            Layout::Auto => {
                let max_line_length = self.config.line_length;
                if !out.try_write(
                    |out| {
                        out.write_horizontal_source_list(
                            self,
                            a,
                            Join(", "),
                            Self::write_indexed_function_arg,
                        )
                    },
                    max_line_length,
                ) {
                    out.write_vertical_source_list(
                        self,
                        a,
                        Join(", "),
                        Self::write_indexed_function_arg,
                    )
                }
            }
        }
    }

    fn write_integer_type<W: Write, F: FnOnce(&mut Self, &mut SourceWriter<W>)>(
        &mut self,
        out: &mut SourceWriter<W>,
        jna_integer_type: &JnaIntegerType,
        extra: F,
    ) {
        let size = jna_integer_type.underlying_jna_integer_type.size();
        let unsigned = !jna_integer_type.signed;
        out.new_line_if_not_start();
        self.write_documentation(out, jna_integer_type.documentation);
        self.write_deprecated(out, jna_integer_type.deprecated);
        write!(out, "class {} extends IntegerType", jna_integer_type.name);
        out.open_brace();
        write!(out, "public {}()", jna_integer_type.name);
        out.open_brace();
        write!(out, "super({size}, {unsigned});");
        out.close_brace(false);
        out.new_line();
        out.new_line();
        write!(out, "public {}(long value)", jna_integer_type.name);
        out.open_brace();
        write!(out, "super({size}, value, {unsigned});");
        out.close_brace(false);
        out.new_line();
        out.new_line();
        write!(out, "public {}(Pointer p)", jna_integer_type.name);
        out.open_brace();
        write!(
            out,
            "this({});",
            jna_integer_type.underlying_jna_integer_type.get_method(),
        );
        out.close_brace(false);
        out.new_line();
        extra(self, out);
        out.close_brace(false);
        out.new_line();
        out.new_line();

        write!(
            out,
            "class {}ByReference extends ByReference",
            jna_integer_type.name
        );
        out.open_brace();
        write!(out, "public {}ByReference()", jna_integer_type.name);
        out.open_brace();
        write!(out, "super({size});");
        out.close_brace(false);
        out.new_line();
        out.new_line();
        write!(
            out,
            "public {}ByReference(Pointer p)",
            jna_integer_type.name
        );
        out.open_brace();
        write!(out, "super({size});");
        out.new_line();
        out.write("setPointer(p);");
        out.close_brace(false);
        out.new_line();
        out.new_line();
        write!(out, "public {} getValue()", jna_integer_type.name);
        out.open_brace();
        write!(out, "Pointer p = getPointer();");
        out.new_line();
        write!(
            out,
            "return new {}({});",
            jna_integer_type.name,
            jna_integer_type.underlying_jna_integer_type.get_method()
        );
        out.close_brace(false);
        out.new_line();
        out.new_line();
        write!(out, "public void setValue({} value)", jna_integer_type.name);
        out.open_brace();
        write!(out, "Pointer p = getPointer();");
        out.new_line();
        write!(
            out,
            "{}",
            jna_integer_type.underlying_jna_integer_type.set_method()
        );
        out.close_brace(false);
        out.new_line();
        out.close_brace(false);
        out.new_line();
    }

    fn write_pointer_type<W: Write>(
        &mut self,
        out: &mut SourceWriter<W>,
        documentation: &Documentation,
        deprecated: Option<&str>,
        name: &str,
    ) {
        self.write_documentation(out, documentation);
        self.write_deprecated(out, deprecated);
        write!(out, "class {} extends PointerType", name);
        out.open_brace();
        write!(out, "public {}()", name);
        out.open_brace();
        out.write("super(null);");
        out.close_brace(false);
        out.new_line();
        write!(out, "public {}(Pointer p)", name);
        out.open_brace();
        out.write("super(p);");
        out.close_brace(false);
        out.close_brace(false);
        out.new_line();
        out.new_line();
        self.write_documentation(out, documentation);
        self.write_deprecated(out, deprecated);
        write!(out, "class {}ByReference extends {}", name, name,);
        out.open_brace();
        write!(out, "public {}ByReference()", name);
        out.open_brace();
        out.write("super(null);");
        out.close_brace(false);
        out.new_line();
        write!(out, "public {}ByReference(Pointer p)", name);
        out.open_brace();
        out.write("super(p);");
        out.close_brace(false);
        out.close_brace(false);
    }
}

pub(crate) fn wrap_java_value(literal: &Literal, ty: &Type) -> Literal {
    match literal {
        Literal::Expr(expr) => match ty {
            Type::Primitive(primitive) => match primitive {
                PrimitiveType::Double => Literal::Expr(format!("{expr}d")),
                PrimitiveType::Float => Literal::Expr(format!("{expr}f")),
                PrimitiveType::Integer {
                    kind: IntKind::LongLong | IntKind::B64,
                    ..
                } => Literal::Expr(format!("{expr}L")),
                PrimitiveType::Integer {
                    kind: IntKind::Long | IntKind::Size | IntKind::SizeT,
                    ..
                } => Literal::Expr(format!("new NativeLong({expr})")),

                _ => literal.clone(),
            },
            Type::Path(path) => Literal::Expr(format!("new {}({expr})", path.export_name())),
            _ => literal.clone(),
        },
        _ => literal.clone(),
    }
}

pub(crate) fn java_writable_literal(ty: &Type, literal: &Literal) -> bool {
    // quite crude for now
    match literal {
        Literal::Expr(e) => {
            !((ty == &Type::Primitive(PrimitiveType::Char32) && e.starts_with("U'\\U"))
                || (matches!(ty, Type::Primitive(PrimitiveType::Integer { .. }))
                    || e.ends_with("ull")))
        }
        _ => false,
    }
}

fn not_implemented<T: Debug, F: Write>(value: &T, out: &mut SourceWriter<F>) {
    write!(out, "/* Not implemented yet : {value:?} */")
}
