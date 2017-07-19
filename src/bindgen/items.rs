/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;
use std::io::Write;
use std::fmt;

use syn;

use bindgen::cdecl;
use bindgen::config::{Config, Language, Layout};
use bindgen::annotation::*;
use bindgen::library::*;
use bindgen::mangle::*;
use bindgen::rename::*;
use bindgen::utilities::*;
use bindgen::writer::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    ISize,
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
            "isize" => Some(PrimitiveType::ISize),
            "i8" => Some(PrimitiveType::Int8),
            "i16" => Some(PrimitiveType::Int16),
            "i32" => Some(PrimitiveType::Int32),
            "i64" => Some(PrimitiveType::Int64),
            "f32" => Some(PrimitiveType::Float),
            "f64" => Some(PrimitiveType::Double),
            _ => None,
        }
    }

    pub fn to_repr_rust(&self) -> &'static str {
        match self {
            &PrimitiveType::Void => "c_void",
            &PrimitiveType::Char => "c_char",
            &PrimitiveType::SChar => "c_schar",
            &PrimitiveType::UChar => "c_uchar",
            &PrimitiveType::Short => "c_short",
            &PrimitiveType::Int => "c_int",
            &PrimitiveType::Long => "c_long",
            &PrimitiveType::LongLong => "c_longlong",
            &PrimitiveType::UShort => "c_ushort",
            &PrimitiveType::UInt => "c_uint",
            &PrimitiveType::ULong => "c_ulong",
            &PrimitiveType::ULongLong => "c_ulonglong",
            &PrimitiveType::WChar => "char",
            &PrimitiveType::Bool => "bool",
            &PrimitiveType::USize => "usize",
            &PrimitiveType::UInt8 => "u8",
            &PrimitiveType::UInt16 => "u16",
            &PrimitiveType::UInt32 => "u32",
            &PrimitiveType::UInt64 => "u64",
            &PrimitiveType::ISize => "isize",
            &PrimitiveType::Int8 => "i8",
            &PrimitiveType::Int16 => "i16",
            &PrimitiveType::Int32 => "i32",
            &PrimitiveType::Int64 => "i64",
            &PrimitiveType::Float => "f32",
            &PrimitiveType::Double => "f64",
        }
    }

    pub fn to_repr_c(&self) -> &'static str {
        match self {
            &PrimitiveType::Void => "void",
            &PrimitiveType::Bool => "bool",
            &PrimitiveType::Char => "char",
            &PrimitiveType::WChar => "wchar_t",
            &PrimitiveType::SChar => "signed char",
            &PrimitiveType::UChar => "unsigned char",
            &PrimitiveType::Short => "short",
            &PrimitiveType::Int => "int",
            &PrimitiveType::Long => "long",
            &PrimitiveType::LongLong => "long long",
            &PrimitiveType::UShort => "unsigned short",
            &PrimitiveType::UInt => "unsigned int",
            &PrimitiveType::ULong => "unsigned long",
            &PrimitiveType::ULongLong => "unsigned long long",
            &PrimitiveType::USize => "size_t",
            &PrimitiveType::UInt8 => "uint8_t",
            &PrimitiveType::UInt16 => "uint16_t",
            &PrimitiveType::UInt32 => "uint32_t",
            &PrimitiveType::UInt64 => "uint64_t",
            &PrimitiveType::ISize => "ssize_t",
            &PrimitiveType::Int8 => "int8_t",
            &PrimitiveType::Int16 => "int16_t",
            &PrimitiveType::Int32 => "int32_t",
            &PrimitiveType::Int64 => "int64_t",
            &PrimitiveType::Float => "float",
            &PrimitiveType::Double => "double",
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
        write!(f, "{}", self.to_repr_c())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    ConstPtr(Box<Type>),
    Ptr(Box<Type>),
    Path(PathRef, Vec<Type>),
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
            &syn::Ty::Path(_, ref path) => {
                let (name, generics) = path.convert_to_generic_single_segment()?;

                if name == "PhantomData" {
                    return Ok(None);
                }

                if let Some(prim) = PrimitiveType::maybe(&name) {
                    if generics.len() > 0 {
                        return Err(format!("primitive has generics"));
                    }
                    Type::Primitive(prim)
                } else {
                    Type::Path(name, generics)
                }
            }
            &syn::Ty::Array(ref ty, syn::ConstExpr::Lit(syn::Lit::Int(size, _))) => {
                let converted = Type::load(ty)?;

                let converted = match converted {
                    Some(converted) => converted,
                    None => return Err(format!("cannot have an array of zero sized types")),
                };

                Type::Array(Box::new(converted), size)
            },
            &syn::Ty::BareFn(ref function) => {
                let args = function.inputs.iter()
                                          .try_skip_map(|x| Type::load(&x.ty))?;
                let ret = function.output.as_type()?;

                Type::FuncPtr(Box::new(ret), args)
            },
            &syn::Ty::Tup(ref fields) => {
                if fields.len() == 0 {
                    return Ok(None);
                }
                return Err(format!("tuples are not supported as types"))
            }
            _ => return Err(format!("unexpected type")),
        };

        return Ok(Some(converted));
    }

    pub fn specialize(&self, mappings: &Vec<(&String, &Type)>) -> Type {
        match self {
            &Type::ConstPtr(ref ty) => {
                Type::ConstPtr(Box::new(ty.specialize(mappings)))
            }
            &Type::Ptr(ref ty) => {
                Type::Ptr(Box::new(ty.specialize(mappings)))
            }
            &Type::Path(ref path, ref generic_values) => {
                for &(param, value) in mappings {
                    if *path == *param {
                        return value.clone();
                    }
                }

                Type::Path(path.clone(),
                           generic_values.iter()
                                         .map(|x| x.specialize(mappings))
                                         .collect())
            }
            &Type::Primitive(ref primitive) => {
                Type::Primitive(primitive.clone())
            }
            &Type::Array(ref ty, ref size) => {
                Type::Array(Box::new(ty.specialize(mappings)), *size)
            }
            &Type::FuncPtr(ref ret, ref args) => {
                Type::FuncPtr(Box::new(ret.specialize(mappings)),
                              args.iter()
                                  .map(|x| x.specialize(mappings))
                                  .collect())
            }
        }
    }

    pub fn add_deps_with_generics(&self, generic_params: &Vec<String>, library: &Library, out: &mut DependencyList) {
        match self {
            &Type::ConstPtr(ref ty) => {
                ty.add_deps_with_generics(generic_params, library, out);
            }
            &Type::Ptr(ref ty) => {
                ty.add_deps_with_generics(generic_params, library, out);
            }
            &Type::Path(ref path, ref generic_values) => {
                if !generic_params.contains(path) {
                    if let Some(value) = library.resolve_path(path) {
                        if !out.items.contains(path) {
                            out.items.insert(path.clone());

                            value.add_deps(library, out);

                            out.order.push(value);
                        }
                    } else {
                        warn!("can't find {}", path);
                    }
                }
                for generic_value in generic_values {
                    generic_value.add_deps_with_generics(generic_params, library, out);
                }
            }
            &Type::Primitive(_) => { }
            &Type::Array(ref ty, _) => {
                ty.add_deps_with_generics(generic_params, library, out);
            }
            &Type::FuncPtr(ref ret, ref args) => {
                ret.add_deps_with_generics(generic_params, library, out);
                for arg in args {
                    arg.add_deps_with_generics(generic_params, library, out);
                }
            }
        }
    }

    pub fn add_deps(&self, library: &Library, out: &mut DependencyList) {
        self.add_deps_with_generics(&Vec::new(), library, out)
    }

    pub fn add_monomorphs(&self, library: &Library, out: &mut Monomorphs) {
        match self {
            &Type::ConstPtr(ref ty) => {
                ty.add_monomorphs(library, out);
            }
            &Type::Ptr(ref ty) => {
                ty.add_monomorphs(library, out);
            }
            &Type::Path(ref path, ref generic_values) => {
                let item = library.resolve_path(path);
                if let Some(item) = item {
                    match item {
                        PathValue::Struct(ref x) => {
                            x.add_monomorphs(library, generic_values, out);
                        },
                        PathValue::OpaqueItem(ref x) => {
                            x.add_monomorphs(generic_values, out);
                        },
                        PathValue::Typedef(ref x) => {
                            assert!(generic_values.len() == 0);
                            x.add_monomorphs(library, out);
                        },
                        PathValue::Specialization(..) => { unreachable!() },
                        _ => { }
                    }
                }
            }
            &Type::Primitive(_) => { }
            &Type::Array(ref ty, _) => {
                ty.add_monomorphs(library, out);
            }
            &Type::FuncPtr(ref ret, ref args) => {
                ret.add_monomorphs(library, out);
                for arg in args {
                    arg.add_monomorphs(library, out);
                }
            }
        }
    }

    pub fn add_specializations(&self, library: &Library, out: &mut SpecializationList) {
        match self {
            &Type::ConstPtr(ref ty) => {
                ty.add_specializations(library, out);
            }
            &Type::Ptr(ref ty) => {
                ty.add_specializations(library, out);
            }
            &Type::Path(ref path, ref generic_values) => {
                let item = library.resolve_path(path);
                if let Some(item) = item {
                    match item {
                        PathValue::Struct(ref x) => {
                            x.add_specializations(library, out);
                        },
                        PathValue::Typedef(ref x) => {
                            assert!(generic_values.len() == 0);
                            x.add_specializations(library, out);
                        },
                        PathValue::Specialization(ref x) => {
                            x.add_specializations(library, out);
                        },
                        _ => { }
                    }
                }
            }
            &Type::Primitive(_) => { }
            &Type::Array(ref ty, _) => {
                ty.add_specializations(library, out);
            }
            &Type::FuncPtr(ref ret, ref args) => {
                ret.add_specializations(library, out);
                for arg in args {
                    arg.add_specializations(library, out);
                }
            }
        }
    }

    pub fn mangle_paths(&mut self, monomorphs: &Monomorphs) {
        match self {
            &mut Type::ConstPtr(ref mut ty) => {
                ty.mangle_paths(monomorphs);
            }
            &mut Type::Ptr(ref mut ty) => {
                ty.mangle_paths(monomorphs);
            }
            &mut Type::Path(ref mut path, ref mut generic_values) => {
                // TODO: simplify
                if generic_values.len() != 0 {
                    if let Some(monomorph_list) = monomorphs.get(path) {
                        if let Some(monomorph) = monomorph_list.get(generic_values) {
                            *path = monomorph.name().to_owned();
                            *generic_values = Vec::new();
                        } else {
                            warn!("cannot find a monomorph for {}::{:?}", path, generic_values);
                        }
                    } else {
                        warn!("cannot find a monomorph for {}::{:?}", path, generic_values);
                    }
                }
            }
            &mut Type::Primitive(_) => { }
            &mut Type::Array(ref mut ty, _) => {
                ty.mangle_paths(monomorphs);
            }
            &mut Type::FuncPtr(ref mut ret, ref mut args) => {
                ret.mangle_paths(monomorphs);
                for arg in args {
                    arg.mangle_paths(monomorphs);
                }
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

impl Source for Type {
    fn write<F: Write>(&self, _config: &Config, out: &mut SourceWriter<F>) {
        cdecl::write_type(out, &self);
    }
}

impl Source for (String, Type) {
    fn write<F: Write>(&self, _config: &Config, out: &mut SourceWriter<F>) {
        cdecl::write_field(out, &self.1, &self.0);
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

    pub fn add_deps(&self, library: &Library, out: &mut DependencyList) {
        self.ret.add_deps(library, out);
        for &(_, ref ty) in &self.args {
            ty.add_deps(library, out);
        }
    }

    pub fn add_monomorphs(&self, library: &Library, out: &mut Monomorphs) {
        self.ret.add_monomorphs(library, out);
        for &(_, ref ty) in &self.args {
            ty.add_monomorphs(library, out);
        }
    }

    pub fn add_specializations(&self, library: &Library, out: &mut SpecializationList) {
        self.ret.add_specializations(library, out);
        for &(_, ref ty) in &self.args {
            ty.add_specializations(library, out);
        }
    }

    pub fn rename_args(&mut self, config: &Config) {
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

    pub fn mangle_paths(&mut self, monomorphs: &Monomorphs) {
        for &mut (_, ref mut ty) in &mut self.args {
            ty.mangle_paths(monomorphs);
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
            cdecl::write_func(out, &func, false);
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
            cdecl::write_func(out, &func, true);
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
    pub generic_params: Vec<String>,
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

    pub fn add_deps(&self, library: &Library, out: &mut DependencyList) {
        for &(_, ref ty) in &self.fields {
            ty.add_deps_with_generics(&self.generic_params, library, out);
        }
    }

    pub fn add_monomorphs(&self, library: &Library, generic_values: &Vec<Type>, out: &mut Monomorphs) {
        assert!(self.generic_params.len() == generic_values.len());

        if self.generic_params.len() == 0 {
            for &(_, ref ty) in &self.fields {
                ty.add_monomorphs(library, out);
            }
            return;
        }

        let mappings = self.generic_params.iter()
                                          .zip(generic_values.iter())
                                          .collect::<Vec<_>>();

        let monomorph = Struct {
            name: mangle_path(&self.name, generic_values),
            annotations: self.annotations.clone(),
            fields: self.fields.iter()
                               .map(|x| (x.0.clone(), x.1.specialize(&mappings)))
                               .collect(),
            generic_params: vec![],
        };

        for &(_, ref ty) in &monomorph.fields {
            ty.add_monomorphs(library, out);
        }

        if !out.contains_key(&self.name) {
            out.insert(self.name.clone(), HashMap::new());
        }
        out.get_mut(&self.name).unwrap().insert(generic_values.clone(), 
                                                Monomorph::Struct(monomorph));
    }

    pub fn add_specializations(&self, library: &Library, out: &mut SpecializationList) {
        for &(_, ref ty) in &self.fields {
            ty.add_specializations(library, out);
        }
    }

    pub fn rename_fields(&mut self, config: &Config) {
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

    pub fn mangle_paths(&mut self, monomorphs: &Monomorphs) {
        for &mut (_, ref mut ty) in &mut self.fields {
            ty.mangle_paths(monomorphs);
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
pub struct OpaqueItem {
    pub name: PathRef,
    pub generic_params: Vec<String>,
    pub annotations: AnnotationSet,
}

impl OpaqueItem {
    pub fn new(name: String,
               generics: &syn::Generics,
               annotations: AnnotationSet) -> OpaqueItem {
        let generic_params = generics.ty_params.iter()
                                               .map(|x| x.ident.to_string())
                                               .collect::<Vec<_>>();

        OpaqueItem {
            name: name,
            generic_params: generic_params,
            annotations: annotations,
        }
    }

    pub fn add_monomorphs(&self, generic_values: &Vec<Type>, out: &mut Monomorphs) {
        assert!(self.generic_params.len() == generic_values.len());

        if self.generic_params.len() == 0 {
            return;
        }

        let monomorph = OpaqueItem {
            name: mangle_path(&self.name, generic_values),
            generic_params: vec![],
            annotations: self.annotations.clone(),
        };

        if !out.contains_key(&self.name) {
            out.insert(self.name.clone(), HashMap::new());
        }
        out.get_mut(&self.name).unwrap().insert(generic_values.clone(), 
                                                Monomorph::OpaqueItem(monomorph));
    }
}

impl Source for OpaqueItem {
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

    pub fn rename_fields(&mut self, config: &Config) {
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
    pub generic_params: Vec<String>,
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

    pub fn add_specializations(&self, library: &Library, out: &mut SpecializationList) {
        match self.specialize(library) {
            Ok(Some(specialization)) => {
                if !out.items.contains(specialization.name()) {
                    out.items.insert(specialization.name().to_owned());

                    specialization.add_specializations(library, out);

                    out.order.push(specialization);
                }
            }
            Ok(None) => { }
            Err(msg) => {
                out.errors.push((self.name.clone(), msg));
            }
        }
    }

    pub fn specialize(&self, library: &Library) -> Result<Option<PathValue>, String> {
        match library.resolve_path(&self.aliased) {
            Some(aliased) => {
                match aliased {
                    PathValue::OpaqueItem(ref aliased) => {
                        if self.generic_values.len() !=
                           aliased.generic_params.len() {
                            return Err(format!("incomplete specialization"));
                        }

                        Ok(Some(PathValue::OpaqueItem(OpaqueItem {
                            name: self.name.clone(),
                            generic_params: self.generic_params.clone(),
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
                            generic_params: self.generic_params.clone(),
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
                            generic_params: self.generic_params.clone(),
                            generic_values: generic_values,
                        }.specialize(library)
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

    pub fn add_deps(&self, library: &Library, out: &mut DependencyList) {
        self.aliased.add_deps(library, out);
    }

    pub fn add_monomorphs(&self, library: &Library, out: &mut Monomorphs) {
        self.aliased.add_monomorphs(library, out);
    }

    pub fn add_specializations(&self, library: &Library, out: &mut SpecializationList) {
        self.aliased.add_specializations(library, out);
    }

    pub fn mangle_paths(&mut self, monomorphs: &Monomorphs) {
        self.aliased.mangle_paths(monomorphs);
    }
}

impl Source for Typedef {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        out.write("typedef ");
        (self.name.clone(), self.aliased.clone()).write(config, out);
        out.write(";");
    }
}
