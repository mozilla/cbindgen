/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;
use std::fmt;

use syn;

use bindgen::cdecl;
use bindgen::config::Config;
use bindgen::library::*;
use bindgen::utilities::*;
use bindgen::writer::*;
use bindgen::ir::Documentation;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    pub fn maybe(path: &str) -> Option<PrimitiveType> {
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
            &PrimitiveType::ISize => "intptr_t",
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

    pub fn get_root_path(&self) -> Option<PathRef> {
        let mut current = self;

        loop {
            match current {
                &Type::ConstPtr(ref ty) => { current = ty },
                &Type::Ptr(ref ty) => { current = ty },
                &Type::Path(ref path, ..) => {
                    return Some(path.clone());
                },
                &Type::Primitive(..) => {
                    return None;
                },
                &Type::Array(..) => {
                    return None;
                },
                &Type::FuncPtr(..) => {
                    return None;
                },
            };
        };
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

    pub fn lookup(&self, library: &Library) -> Vec<PathValue> {
        match *self {
            Type::ConstPtr(ref ty)|
            Type::Ptr(ref ty)|
            Type::Array(ref ty, _) => ty.lookup(library),
            Type::FuncPtr(ref ret, ref args) => {
                let mut ret = ret.lookup(library);
                for arg in args {
                    ret.extend_from_slice(&arg.lookup(library));
                }
                ret
            }
            Type::Primitive(_) => Vec::new(),
            Type::Path(ref path, _) => {
                library.resolve_path(path)
                    .map(|p| vec![p])
                    .unwrap_or_else(Vec::new)
            }
        }
    }

    pub fn resolve_monomorphed_type(&self, monomorphs: &MonomorphList) -> Option<String> {
        match *self {
            Type::Path(_, ref generics) => {
                if let Some(s) = monomorphs.get(generics) {
                    Some(s.name().clone().to_owned())
                }else {
                    None
                }
            }
            Type::ConstPtr(ref ty) | Type::Ptr(ref ty) | Type::Array(ref ty, _) => {
                ty.resolve_monomorphed_type(monomorphs)
            }
            ref o => {
                println!("\t\t{:?}", o);
                None
            }
        }
    }

    pub fn add_deps_with_generics(&self,
                                  generic_params: &Vec<String>,
                                  library: &Library,
                                  out: &mut DependencyList) {
        match self {
            &Type::ConstPtr(ref ty) => {
                ty.add_deps_with_generics(generic_params, library, out);
            }
            &Type::Ptr(ref ty) => {
                ty.add_deps_with_generics(generic_params, library, out);
            }
            &Type::Path(ref path, ref generic_values) => {
                for generic_value in generic_values {
                    generic_value.add_deps_with_generics(generic_params, library, out);
                }
                if !generic_params.contains(path) {
                    if let Some(value) = library.resolve_path(path) {

                        if !out.lookup.contains(path) {
                            out.lookup.insert(path.clone());
                            value.add_deps(library, out);
                            out.items.push(value);
                        }
                    } else {
                        warn!("can't find {}", path);
                    }
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

    pub fn add_monomorphs(&self, library: &Library,
                          out: &mut Monomorphs,
                          cycle_check: &mut CycleCheckList)
    {
        if !cycle_check.contains(self){
            cycle_check.insert(self.clone());
                match self {
                    &Type::ConstPtr(ref ty) => {
                        ty.add_monomorphs(library, out, cycle_check);
                    }
                    &Type::Ptr(ref ty) => {
                        ty.add_monomorphs(library, out, cycle_check);
                    }
                    &Type::Path(ref path, ref generic_values) => {
                        let item = library.resolve_path(path);
                        if let Some(item) = item {
                            match item {
                                PathValue::Struct(ref x) => {
                                    x.add_monomorphs(library, generic_values, out, cycle_check);
                                }
                                PathValue::OpaqueItem(ref x) => {
                                    x.add_monomorphs(generic_values, out);
                                }
                                PathValue::Typedef(ref x) => {
                                    assert!(generic_values.len() == 0);
                                    x.add_monomorphs(library, out, cycle_check);
                                }
                                PathValue::Specialization(..) => unreachable!(),
                                _ => {}
                            }
                        }
                    }
                    &Type::Primitive(_) => {}
                    &Type::Array(ref ty, _) => {
                        ty.add_monomorphs(library, out, cycle_check);
                    }
                    &Type::FuncPtr(ref ret, ref args) => {
                        ret.add_monomorphs(library, out, cycle_check);
                        for arg in args {
                            arg.add_monomorphs(library, out, cycle_check);
                        }
                    }
                }
        }
    }

    pub fn add_specializations(&self, library: &Library,
                               out: &mut SpecializationList,
                               cycle_check: &mut CycleCheckList)
    {
        if !cycle_check.contains(self) {
            cycle_check.insert(self.clone());
            match self {
                &Type::ConstPtr(ref ty) => {
                    ty.add_specializations(library, out, cycle_check);
                }
                &Type::Ptr(ref ty) => {
                    ty.add_specializations(library, out, cycle_check);
                }
                &Type::Path(ref path, ref generic_values) => {
                    if let Some(item) = library.resolve_path(path) {
                        match item {
                            PathValue::Struct(ref x) => {
                                x.add_specializations(library, out, cycle_check);
                            }
                            PathValue::Typedef(ref x) => {
                                assert!(generic_values.len() == 0);
                                x.add_specializations(library, out, cycle_check);
                            }
                            PathValue::Specialization(ref x) => {
                                x.add_specializations(library, out, cycle_check);
                            }
                            _ => {}
                        }
                    }
                }
                &Type::Primitive(_) => {}
                &Type::Array(ref ty, _) => {
                    ty.add_specializations(library, out, cycle_check);
                }
                &Type::FuncPtr(ref ret, ref args) => {
                    ret.add_specializations(library, out, cycle_check);
                    for arg in args {
                        arg.add_specializations(library, out, cycle_check);
                    }
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

    pub fn can_cmp_order(&self) -> bool {
        match self {
            &Type::ConstPtr(..) => true,
            &Type::Ptr(..) => true,
            &Type::Path(..) => true,
            &Type::Primitive(ref p) => p.can_cmp_order(),
            &Type::Array(..) => false,
            &Type::FuncPtr(..) => false,
        }
    }

    pub fn can_cmp_eq(&self) -> bool {
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

impl Source for (String, Type, Documentation) {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        self.2.write(config, out);
        cdecl::write_field(out, &self.1, &self.0);
    }
}

pub trait SynFnRetTyHelpers {
    fn as_type(&self) -> Result<Type, String>;
}

impl SynFnRetTyHelpers for syn::FunctionRetTy {
    fn as_type(&self) -> Result<Type, String> {
        match self {
            &syn::FunctionRetTy::Default => Ok(Type::Primitive(PrimitiveType::Void)),
            &syn::FunctionRetTy::Ty(ref t) => {
                if let Some(x) = Type::load(t)? {
                    Ok(x)
                } else {
                    Ok(Type::Primitive(PrimitiveType::Void))
                }
            },
        }
    }
}
