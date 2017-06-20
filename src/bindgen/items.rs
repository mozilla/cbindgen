use std::io::Write;
use std::fmt;

use syn;

use bindgen::cdecl;
use bindgen::config::{Config, Language, Layout};
use bindgen::annotation::*;
use bindgen::library::*;
use bindgen::rename::*;
use bindgen::utilities::*;
use bindgen::writer::*;

#[derive(Debug, Clone)]
pub enum PrimitiveType {
    Void,
    Bool,
    Char,
    WChar,
    SChar,
    UChar,
    Short,
    Int,
    Long,
    LongLong,
    UShort,
    UInt,
    ULong,
    ULongLong,
    USize,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Int8,
    Int16,
    Int32,
    Int64,
    Float,
    Double,
}

impl PrimitiveType {
    fn maybe(path: &str) -> Option<PrimitiveType> {
        match path {
            "c_void" => Some(PrimitiveType::Void),
            "c_char" => Some(PrimitiveType::Char),
            "c_schar" => Some(PrimitiveType::SChar),
            "c_uchar" => Some(PrimitiveType::UChar),
            "c_float" => Some(PrimitiveType::Float),
            "c_double" => Some(PrimitiveType::Double),
            "c_short" => Some(PrimitiveType::Short),
            "c_int" => Some(PrimitiveType::Int),
            "c_long" => Some(PrimitiveType::Long),
            "c_longlong" => Some(PrimitiveType::LongLong),
            "c_ushort" => Some(PrimitiveType::UShort),
            "c_uint" => Some(PrimitiveType::UInt),
            "c_ulong" => Some(PrimitiveType::ULong),
            "c_ulonglong" => Some(PrimitiveType::ULongLong),
            "bool" => Some(PrimitiveType::Bool),
            "char" => Some(PrimitiveType::WChar),
            "usize" => Some(PrimitiveType::USize),
            "u8" => Some(PrimitiveType::UInt8),
            "u16" => Some(PrimitiveType::UInt16),
            "u32" => Some(PrimitiveType::UInt32),
            "u64" => Some(PrimitiveType::UInt64),
            "i8" => Some(PrimitiveType::Int8),
            "i16" => Some(PrimitiveType::Int16),
            "i32" => Some(PrimitiveType::Int32),
            "i64" => Some(PrimitiveType::Int64),
            "f32" => Some(PrimitiveType::Float),
            "f64" => Some(PrimitiveType::Double),
            _ => None,
        }
    }

    fn can_cmp_order(&self) -> bool {
        match self {
            &PrimitiveType::Bool => false,
            _ => true,
        }
    }
    fn can_cmp_eq(&self) -> bool {
        true
    }
}

impl fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &PrimitiveType::Void => write!(f, "void"),
            &PrimitiveType::Bool => write!(f, "bool"),
            &PrimitiveType::Char => write!(f, "char"),
            &PrimitiveType::WChar => write!(f, "wchar_t"),
            &PrimitiveType::SChar => write!(f, "signed char"),
            &PrimitiveType::UChar => write!(f, "unsigned char"),
            &PrimitiveType::Short => write!(f, "short"),
            &PrimitiveType::Int => write!(f, "int"),
            &PrimitiveType::Long => write!(f, "long"),
            &PrimitiveType::LongLong => write!(f, "long long"),
            &PrimitiveType::UShort => write!(f, "unsigned short"),
            &PrimitiveType::UInt => write!(f, "unsigned int"),
            &PrimitiveType::ULong => write!(f, "unsigned long"),
            &PrimitiveType::ULongLong => write!(f, "unsigned long long"),
            &PrimitiveType::USize => write!(f, "size_t"),
            &PrimitiveType::UInt8 => write!(f, "uint8_t"),
            &PrimitiveType::UInt16 => write!(f, "uint16_t"),
            &PrimitiveType::UInt32 => write!(f, "uint32_t"),
            &PrimitiveType::UInt64 => write!(f, "uint64_t"),
            &PrimitiveType::Int8 => write!(f, "int8_t"),
            &PrimitiveType::Int16 => write!(f, "int16_t"),
            &PrimitiveType::Int32 => write!(f, "int32_t"),
            &PrimitiveType::Int64 => write!(f, "int64_t"),
            &PrimitiveType::Float => write!(f, "float"),
            &PrimitiveType::Double => write!(f, "double")
        }
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    ConstPtr(Box<Type>),
    Ptr(Box<Type>),
    Path(PathRef),
    Primitive(PrimitiveType),
    Array(Box<Type>, u64),
    FuncPtr(Box<Type>, Vec<Type>),
}

impl Type {
    pub fn load(ty: &syn::Ty) -> Result<Option<Type>, String> {
        let converted = match ty {
            &syn::Ty::Rptr(_, ref mut_ty) => {
                let converted = Type::load(&mut_ty.ty)?;

                let converted = match converted {
                    Some(converted) => converted,
                    None => return Err(format!("cannot have a pointer to a zero sized type")),
                };

                match mut_ty.mutability {
                    syn::Mutability::Mutable => Type::Ptr(Box::new(converted)),
                    syn::Mutability::Immutable => Type::ConstPtr(Box::new(converted)),
                }
            }
            &syn::Ty::Ptr(ref mut_ty) => {
                let converted = Type::load(&mut_ty.ty)?;

                let converted = match converted {
                    Some(converted) => converted,
                    None => return Err(format!("cannot have a pointer to a zero sized type")),
                };

                match mut_ty.mutability {
                    syn::Mutability::Mutable => Type::Ptr(Box::new(converted)),
                    syn::Mutability::Immutable => Type::ConstPtr(Box::new(converted)),
                }
            }
            &syn::Ty::Path(_, ref p) => {
                let (name, generics) = p.convert_to_generic_single_segment()?;

                if name == "PhantomData" {
                    return Ok(None);
                } else if generics.len() != 0 {
                    return Err(format!("cannot have a type with generics"));
                } else {
                    if let Some(prim) = PrimitiveType::maybe(&name) {
                        Type::Primitive(prim)
                    } else {
                        Type::Path(name)
                    }
                }
            }
            &syn::Ty::Array(ref ty, syn::ConstExpr::Lit(syn::Lit::Int(sz, _))) => {
                let converted = Type::load(ty)?;

                let converted = match converted {
                    Some(converted) => converted,
                    None => return Err(format!("cannot have an array of zero sized types")),
                };

                Type::Array(Box::new(converted), sz)
            },
            &syn::Ty::BareFn(ref f) => {
                let args = f.inputs.iter()
                                   .try_skip_map(|x| Type::load(&x.ty))?;
                let ret = f.output.as_type()?;

                Type::FuncPtr(Box::new(ret), args)
            },
            &syn::Ty::Tup(ref tys) => {
                if tys.len() == 0 {
                    return Ok(None);
                }
                return Err(format!("tuples are not supported as types"))
            }
            _ => return Err(format!("unexpected type")),
        };

        return Ok(Some(converted));
    }

    pub fn add_deps_with_generics(&self, generic_params: &Vec<String>, library: &Library, out: &mut DependencyGraph) {
        match self {
            &Type::ConstPtr(ref t) => {
                t.add_deps(library, out);
            }
            &Type::Ptr(ref t) => {
                t.add_deps(library, out);
            }
            &Type::Path(ref p) => {
                if generic_params.contains(p) {
                    return;
                }
                library.add_deps_for_path(p, out);
            }
            &Type::Primitive(_) => { }
            &Type::Array(ref t, _) => {
                t.add_deps(library, out);
            }
            &Type::FuncPtr(ref ret, ref args) => {
                ret.add_deps(library, out);
                for arg in args {
                    arg.add_deps(library, out);
                }
            }
        }
    }

    pub fn add_deps(&self, library: &Library, out: &mut DependencyGraph) {
        self.add_deps_with_generics(&Vec::new(), library, out)
    }

    pub fn specialize(&self, mappings: &Vec<(&String, &Type)>) -> Type {
        match self {
            &Type::ConstPtr(ref t) => {
                Type::ConstPtr(Box::new(t.specialize(mappings)))
            }
            &Type::Ptr(ref t) => {
                Type::Ptr(Box::new(t.specialize(mappings)))
            }
            &Type::Path(ref p) => {
                for &(param, value) in mappings {
                    if *p == *param {
                        return value.clone();
                    }
                }

                Type::Path(p.clone())
            }
            &Type::Primitive(ref p) => {
                Type::Primitive(p.clone())
            }
            &Type::Array(ref t, ref sz) => {
                Type::Array(Box::new(t.specialize(mappings)), *sz)
            }
            &Type::FuncPtr(ref ret, ref args) => {
                Type::FuncPtr(Box::new(ret.specialize(mappings)),
                              args.iter()
                                  .map(|x| x.specialize(mappings))
                                  .collect())
            }
        }
    }

    fn can_cmp_order(&self) -> bool {
        match self {
            &Type::ConstPtr(..) => true,
            &Type::Ptr(..) => true,
            &Type::Path(..) => true,
            &Type::Primitive(ref p) => p.can_cmp_order(),
            &Type::Array(..) => false,
            &Type::FuncPtr(..) => false,
        }
    }
    fn can_cmp_eq(&self) -> bool {
        match self {
            &Type::ConstPtr(..) => true,
            &Type::Ptr(..) => true,
            &Type::Path(..) => true,
            &Type::Primitive(ref p) => p.can_cmp_eq(),
            &Type::Array(..) => false,
            &Type::FuncPtr(..) => true,
        }
    }
}

impl Source for (String, Type) {
    fn write<F: Write>(&self, _config: &Config, out: &mut SourceWriter<F>) {
        cdecl::write_type(out, &self.1, &self.0);
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub annotations: AnnotationSet,
    pub ret: Type,
    pub args: Vec<(String, Type)>,
    pub extern_decl: bool,
}

impl Function {
    pub fn load(name: String,
                annotations: AnnotationSet,
                decl: &syn::FnDecl,
                extern_decl: bool) -> Result<Function, String>
    {
        let args = decl.inputs.iter()
                              .try_skip_map(|x| x.as_ident_and_type())?;
        let ret = decl.output.as_type()?;

        Ok(Function {
            name: name,
            annotations: annotations,
            ret: ret,
            args: args,
            extern_decl: extern_decl,
        })
    }

    pub fn apply_renaming(&mut self, config: &Config) {
        let rules = [self.annotations.parse_atom::<RenameRule>("rename-all"),
                     config.function.rename_args];

        if let Some(r) = find_first_some(&rules) {
            self.args = self.args.iter()
                                 .map(|x| (r.apply_to_snake_case(&x.0,
                                                                 IdentifierType::FunctionArg),
                                           x.1.clone()))
                                  .collect()
        }
    }

    pub fn add_deps(&self, library: &Library, out: &mut DependencyGraph) {
        self.ret.add_deps(library, out);
        for &(_, ref ty) in &self.args {
            ty.add_deps(library, out);
        }
    }
}

impl Source for Function {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        fn write_1<W: Write>(func: &Function, config: &Config, out: &mut SourceWriter<W>) {
            let prefix = config.function.prefix(&func.annotations);
            let postfix = config.function.postfix(&func.annotations);

            if let Some(ref prefix) = prefix {
                out.write(prefix);
                out.write(" ");
            }
            cdecl::write_func(out, &func);
            if let Some(ref postfix) = postfix {
                out.write(" ");
                out.write(postfix);
            }
            out.write(";");
        }

        fn write_2<W: Write>(func: &Function, config: &Config, out: &mut SourceWriter<W>) {
            let prefix = config.function.prefix(&func.annotations);
            let postfix = config.function.postfix(&func.annotations);

            if let Some(ref prefix) = prefix {
                out.write(prefix);
                out.new_line();
            }
            cdecl::write_func(out, &func);
            if let Some(ref postfix) = postfix {
                out.new_line();
                out.write(postfix);
            }
            out.write(";");
        };

        let option_1 = out.measure(|out| write_1(self, config, out));

        if (config.function.args == Layout::Auto && option_1 <= config.line_length) ||
           config.function.args == Layout::Horizontal {
            write_1(self, config, out);
        } else {
            write_2(self, config, out);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub annotations: AnnotationSet,
    pub fields: Vec<(String, Type)>,
    pub generic_params: Vec<PathRef>,
}

impl Struct {
    pub fn load(name: String,
                annotations: AnnotationSet,
                decl: &syn::VariantData,
                generics: &syn::Generics) -> Result<Struct, String>
    {
        let fields = match decl {
            &syn::VariantData::Struct(ref fields) => {
                fields.iter()
                      .try_skip_map(|x| x.as_ident_and_type())?
            }
            &syn::VariantData::Tuple(ref fields) => {
                let mut out = Vec::new();
                let mut current = 0;
                for field in fields {
                    if let Some(x) = Type::load(&field.ty)? {
                        out.push((format!("{}", current), x));
                        current += 1;
                    }
                }
                out
            }
            &syn::VariantData::Unit => {
                vec![]
            }
        };

        let generic_params = generics.ty_params.iter()
                                               .map(|x| x.ident.to_string())
                                               .collect::<Vec<_>>();

        Ok(Struct {
            name: name,
            annotations: annotations,
            fields: fields,
            generic_params: generic_params,
        })
    }

    pub fn apply_renaming(&mut self, config: &Config) {
        let rules = [self.annotations.parse_atom::<RenameRule>("rename-all"),
                     config.structure.rename_fields];

        if let Some(o) = self.annotations.list("field-names") {
            let mut overriden_fields = Vec::new();

            for (i, &(ref name, ref ty)) in self.fields.iter().enumerate() {
                if i >= o.len() {
                    overriden_fields.push((name.clone(), ty.clone()));
                } else {
                    overriden_fields.push((o[i].clone(), ty.clone()));
                }
            }

            self.fields = overriden_fields;
        } else if let Some(r) = find_first_some(&rules) {
            self.fields = self.fields.iter()
                                     .map(|x| (r.apply_to_snake_case(&x.0,
                                                                     IdentifierType::StructMember),
                                               x.1.clone()))
                                     .collect();
        }
    }

    pub fn add_deps(&self, library: &Library, out: &mut DependencyGraph) {
        for &(_, ref ty) in &self.fields {
            ty.add_deps_with_generics(&self.generic_params, library, out);
        }
    }
}

impl Source for Struct {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        assert!(self.generic_params.is_empty());

        if config.language == Language::C {
            out.write("typedef struct");
        } else {
            out.write(&format!("struct {}", self.name));
        }
        out.open_brace();

        out.write_vertical_source_list(&self.fields, ListType::Cap(";"));

        if config.language == Language::Cxx {
            let mut wrote_start_newline = false;

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

                out.write(&format!("bool operator{}(const {}& {}) const", op, self.name, other));
                out.open_brace();
                out.write("return ");
                out.write_vertical_list(&self.fields.iter()
                                                    .map(|x| format!("{} {} {}.{}", x.0, op, other, x.0))
                                                    .collect(),
                                        ListType::Join(&format!(" {}", conjuc)));
                out.write(";");
                out.close_brace(false);
            };

            if config.structure.derive_eq(&self.annotations) &&
               !self.fields.is_empty() && self.fields.iter().all(|x| x.1.can_cmp_eq()) {
                emit_op("==", "&&");
            }
            if config.structure.derive_neq(&self.annotations) &&
               !self.fields.is_empty() && self.fields.iter().all(|x| x.1.can_cmp_eq()) {
                emit_op("!=", "||");
            }
            if config.structure.derive_lt(&self.annotations) &&
               self.fields.len() == 1 && self.fields[0].1.can_cmp_order() {
                emit_op("<", "&&");
            }
            if config.structure.derive_lte(&self.annotations) &&
               self.fields.len() == 1 && self.fields[0].1.can_cmp_order() {
                emit_op("<=", "&&");
            }
            if config.structure.derive_gt(&self.annotations) &&
               self.fields.len() == 1 && self.fields[0].1.can_cmp_order() {
                emit_op(">", "&&");
            }
            if config.structure.derive_gte(&self.annotations) &&
               self.fields.len() == 1 && self.fields[0].1.can_cmp_order() {
                emit_op(">=", "&&");
            }
        }

        if config.language == Language::C {
            out.close_brace(false);
            out.write(&format!(" {};", self.name));
        } else {
            out.close_brace(true);
        }
    }
}

#[derive(Debug, Clone)]
pub struct OpaqueStruct {
    pub name: PathRef,
    pub annotations: AnnotationSet,
}

impl OpaqueStruct {
    pub fn new(name: String, annotations: AnnotationSet) -> OpaqueStruct
    {
        OpaqueStruct {
            name: name,
            annotations: annotations,
        }
    }
}

impl Source for OpaqueStruct {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        if config.language == Language::C {
            out.write(&format!("struct {};", self.name));
            out.new_line();
            out.write(&format!("typedef struct {} {};", self.name, self.name));
        } else {
            out.write(&format!("struct {};", self.name));
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Repr {
    None,
    C,
    U8,
    U16,
    U32,
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub repr: Repr,
    pub annotations: AnnotationSet,
    pub values: Vec<(String, u64)>,
}

impl Enum {
    pub fn load(name: String,
                repr: Repr,
                annotations: AnnotationSet,
                variants: &Vec<syn::Variant>) -> Result<Enum, String>
    {
        if repr != Repr::U32 &&
           repr != Repr::U16 &&
           repr != Repr::U8 {
            return if repr == Repr::C {
                Err(format!("repr(C) is not FFI safe for enums"))
            } else {
                Err(format!("enum not marked with a repr(u32) or repr(u16) or repr(u8)"))
            };
        }

        let mut values = Vec::new();
        let mut current = 0;

        for variant in variants {
            match variant.data {
                syn::VariantData::Unit => {
                    match variant.discriminant {
                        Some(syn::ConstExpr::Lit(syn::Lit::Int(i, _))) => {
                            current = i;
                        }
                        Some(_) => {
                            return Err(format!("unsupported discriminant"));
                        }
                        None => { /* okay, we just use current */ }
                    }

                    values.push((variant.ident.to_string(), current));
                    current = current + 1;
                }
                _ => {
                    return Err(format!("unsupported variant"));
                }
            }
        }

        if let Some(variants) = annotations.list("enum-trailing-values") {
            for variant in variants {
                values.push((variant, current));
                current = current + 1;
            }
        }

        Ok(Enum {
            name: name,
            repr: repr,
            annotations: annotations,
            values: values,
        })
    }

    pub fn apply_renaming(&mut self, config: &Config) {
        let rules = [self.annotations.parse_atom::<RenameRule>("rename-all"),
                     config.enumeration.rename_variants];

        if let Some(r) = find_first_some(&rules) {
            self.values = self.values.iter()
                                     .map(|x| (r.apply_to_pascal_case(&x.0,
                                                                      IdentifierType::EnumVariant(self)),
                                               x.1.clone()))
                                     .collect();
        }
    }
}

impl Source for Enum {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        let size = match self.repr {
            Repr::U32 => "uint32_t",
            Repr::U16 => "uint16_t",
            Repr::U8 => "uint8_t",
            _ => unreachable!(),
        };

        if config.language == Language::C {
            out.write(&format!("enum {}", self.name));
        } else {
            out.write(&format!("enum class {} : {}", self.name, size));
        }
        out.open_brace();
        for (i, value) in self.values.iter().enumerate() {
            if i != 0 {
                out.new_line()
            }
            out.write(&format!("{} = {},", value.0, value.1));
        }
        if config.enumeration.add_sentinel(&self.annotations) {
            out.new_line();
            out.new_line();
            out.write("Sentinel /* this must be last for serialization purposes. */");
        }
        out.close_brace(true);

        if config.language == Language::C {
            out.new_line();
            out.write(&format!("typedef {} {};", size, self.name));
        }
    }
}

/// A type alias that generates a copy of its aliasee with a new name. If the type
/// alias has generic values, it monomorphosizes its aliasee. This is useful for
/// presenting an interface that includes generic types.
#[derive(Debug, Clone)]
pub struct Specialization {
    pub name: String,
    pub annotations: AnnotationSet,
    pub aliased: PathRef,
    pub generic_params: Vec<PathRef>,
    pub generic_values: Vec<Type>,
}

impl Specialization {
    pub fn load(name: String,
                annotations: AnnotationSet,
                generics: &syn::Generics,
                ty: &syn::Ty) -> Result<Specialization, String>
    {
        match ty {
            &syn::Ty::Path(ref _q, ref p) => {
                let generic_params = generics.ty_params.iter()
                                                       .map(|x| x.ident.to_string())
                                                       .collect::<Vec<_>>();

                let (path, generic_values) = p.convert_to_generic_single_segment()?;

                if PrimitiveType::maybe(&path).is_some() {
                    return Err(format!("can't specialize a primitive"));
                }

                Ok(Specialization {
                    name: name,
                    annotations: annotations,
                    aliased: path,
                    generic_params: generic_params,
                    generic_values: generic_values,
                })
            }
            _ => {
                Err(format!("not a path"))
            }
        }
    }

    pub fn add_deps(&self, library: &Library, out: &mut DependencyGraph) {
        library.add_deps_for_path_deps(&self.aliased, out);
        for value in &self.generic_values {
            value.add_deps_with_generics(&self.generic_params, &library, out);
        }
    }

    pub fn specialize(&self,
                      config: &Config,
                      library: &Library) -> Result<Option<PathValue>, String> {
        if self.generic_params.len() > 0 {
            return Ok(None);
        }

        match library.resolve_path(&self.aliased) {
            Some(aliased) => {
                match aliased {
                    PathValue::OpaqueStruct(_) => {
                        Ok(Some(PathValue::OpaqueStruct(OpaqueStruct {
                            name: self.name.clone(),
                            annotations: self.annotations.clone(),
                        })))
                    }
                    PathValue::Struct(aliased) => {
                        if self.generic_values.len() !=
                           aliased.generic_params.len() {
                            return Err(format!("incomplete specialization"));
                        }

                        let mappings = aliased.generic_params.iter()
                                                             .zip(self.generic_values.iter())
                                                             .collect::<Vec<_>>();

                        Ok(Some(PathValue::Struct(Struct {
                            name: self.name.clone(),
                            annotations: self.annotations.clone(),
                            fields: aliased.fields.iter()
                                                  .map(|x| (x.0.clone(), x.1.specialize(&mappings)))
                                                  .collect(),
                            generic_params: vec![],
                        })))
                    }
                    PathValue::Enum(aliased) => {
                        Ok(Some(PathValue::Enum(Enum {
                            name: self.name.clone(),
                            repr: aliased.repr.clone(),
                            annotations: self.annotations.clone(),
                            values: aliased.values.clone(),
                        })))
                    }
                    PathValue::Typedef(aliased) => {
                        Ok(Some(PathValue::Typedef(Typedef {
                            name: self.name.clone(),
                            annotations: self.annotations.clone(),
                            aliased: aliased.aliased.clone(),
                        })))
                    }
                    PathValue::Specialization(aliased) => {
                        if self.generic_values.len() !=
                           aliased.generic_params.len() {
                            return Err(format!("incomplete specialization"));
                        }

                        let mappings = aliased.generic_params.iter()
                                                             .zip(self.generic_values.iter())
                                                             .collect::<Vec<_>>();

                        let generic_values = aliased.generic_values.iter()
                                                                   .map(|x| x.specialize(&mappings))
                                                                   .collect();

                        Specialization {
                            name: self.name.clone(),
                            annotations: self.annotations.clone(),
                            aliased: aliased.aliased.clone(),
                            generic_params: Vec::new(),
                            generic_values: generic_values,
                        }.specialize(config, library)
                    }
                }
            }
            None => {
                Err(format!("couldn't find aliased type"))
            }
        }
    }
}

/// A type alias that is represented as a C typedef
#[derive(Debug, Clone)]
pub struct Typedef {
    pub name: String,
    pub annotations: AnnotationSet,
    pub aliased: Type,
}

impl Typedef {
    pub fn load(name: String,
                annotations: AnnotationSet,
                ty: &syn::Ty) -> Result<Typedef, String> {
        if let Some(x) = Type::load(ty)? {
            Ok(Typedef {
                name: name,
                annotations: annotations,
                aliased: x,
            })
        } else {
            Err(format!("cannot have a typedef of a zero sized type"))
        }
    }

    pub fn add_deps(&self, library: &Library, out: &mut DependencyGraph) {
        self.aliased.add_deps(library, out);
    }
}

impl Source for Typedef {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        out.write("typedef ");
        (self.name.clone(), self.aliased.clone()).write(config, out);
        out.write(";");
    }
}
