/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;
use std::fmt;

use syn;

use bindgen::cdecl;
use bindgen::config::Config;
use bindgen::dependencies::Dependencies;
use bindgen::ir::{Documentation, GenericParams, GenericPath, Path};
use bindgen::library::Library;
use bindgen::monomorph::Monomorphs;
use bindgen::utilities::IterHelpers;
use bindgen::writer::{Source, SourceWriter};

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
    SizeT,
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
            "size_t" => Some(PrimitiveType::SizeT),
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
            &PrimitiveType::SizeT => "size_t",
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
            &PrimitiveType::SizeT => "size_t",
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
    Path(GenericPath),
    Primitive(PrimitiveType),
    Array(Box<Type>, String),
    FuncPtr(Box<Type>, Vec<Type>),
}

impl Type {
    pub fn load(ty: &syn::Ty) -> Result<Option<Type>, String> {
        let converted = match ty {
            &syn::Ty::Rptr(_, ref mut_ty) => {
                let converted = Type::load(&mut_ty.ty)?;

                let converted = match converted {
                    Some(converted) => converted,
                    None => {
                        return Err(
                            "Cannot have a pointer to a zero sized type. If you are \
                             trying to represent `void*` use `c_void*`."
                                .to_owned(),
                        )
                    }
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
                    None => {
                        return Err(
                            "Cannot have a pointer to a zero sized type. If you are \
                             trying to represent `void*` use `c_void*`."
                                .to_owned(),
                        )
                    }
                };

                match mut_ty.mutability {
                    syn::Mutability::Mutable => Type::Ptr(Box::new(converted)),
                    syn::Mutability::Immutable => Type::ConstPtr(Box::new(converted)),
                }
            }
            &syn::Ty::Path(_, ref path) => {
                let path = GenericPath::load(path)?;

                if path.name == "PhantomData" {
                    return Ok(None);
                }

                if let Some(prim) = PrimitiveType::maybe(&path.name) {
                    if path.generics.len() > 0 {
                        return Err("Primitive has generics.".to_owned());
                    }
                    Type::Primitive(prim)
                } else {
                    Type::Path(path)
                }
            }
            &syn::Ty::Array(ref ty, syn::ConstExpr::Lit(syn::Lit::Int(size, _))) => {
                let converted = Type::load(ty)?;

                let converted = match converted {
                    Some(converted) => converted,
                    None => return Err("Cannot have an array of zero sized types.".to_owned()),
                };

                Type::Array(Box::new(converted), format!("{}", size))
            }
            &syn::Ty::Array(ref ty, syn::ConstExpr::Path(ref path)) => {
                let converted = Type::load(ty)?;

                let converted = match converted {
                    Some(converted) => converted,
                    None => return Err("Cannot have an array of zero sized types.".to_owned()),
                };

                let path = GenericPath::load(path)?;

                Type::Array(Box::new(converted), path.name)
            }
            &syn::Ty::BareFn(ref function) => {
                let args = function.inputs.iter().try_skip_map(|x| Type::load(&x.ty))?;
                let ret = function.output.as_type()?;

                Type::FuncPtr(Box::new(ret), args)
            }
            &syn::Ty::Tup(ref fields) => {
                if fields.len() == 0 {
                    return Ok(None);
                }
                return Err("Tuples are not supported types.".to_owned());
            }
            _ => return Err("Unsupported type.".to_owned()),
        };

        return Ok(Some(converted));
    }

    pub fn is_primitive_or_ptr_primitive(&self) -> bool {
        match self {
            &Type::Primitive(..) => true,
            &Type::ConstPtr(ref x) => match x.as_ref() {
                &Type::Primitive(..) => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn is_repr_ptr(&self) -> bool {
        match self {
            &Type::Ptr(..) => true,
            &Type::ConstPtr(..) => true,
            &Type::FuncPtr(..) => true,
            _ => false,
        }
    }

    pub fn simplify_option_to_ptr(&mut self) {
        let mut simplified = None;

        if let &mut Type::Path(ref mut path) = self {
            if path.name == "Option" && path.generics.len() == 1 && path.generics[0].is_repr_ptr() {
                simplified = Some(path.generics.pop().unwrap());
            }
        }

        if let Some(ty) = simplified {
            *self = ty;
        }
    }

    pub fn get_root_path(&self) -> Option<Path> {
        let mut current = self;
        loop {
            match current {
                &Type::ConstPtr(ref ty) => current = ty,
                &Type::Ptr(ref ty) => current = ty,
                &Type::Path(ref path) => {
                    return Some(path.name.clone());
                }
                &Type::Primitive(..) => {
                    return None;
                }
                &Type::Array(..) => {
                    return None;
                }
                &Type::FuncPtr(..) => {
                    return None;
                }
            };
        }
    }

    pub fn specialize(&self, mappings: &Vec<(&String, &Type)>) -> Type {
        match self {
            &Type::ConstPtr(ref ty) => Type::ConstPtr(Box::new(ty.specialize(mappings))),
            &Type::Ptr(ref ty) => Type::Ptr(Box::new(ty.specialize(mappings))),
            &Type::Path(ref path) => {
                for &(param, value) in mappings {
                    if *path.name == *param {
                        return value.clone();
                    }
                }

                let specialized = GenericPath::new(
                    path.name.clone(),
                    path.generics
                        .iter()
                        .map(|x| x.specialize(mappings))
                        .collect(),
                );
                Type::Path(specialized)
            }
            &Type::Primitive(ref primitive) => Type::Primitive(primitive.clone()),
            &Type::Array(ref ty, ref constant) => {
                Type::Array(Box::new(ty.specialize(mappings)), constant.clone())
            }
            &Type::FuncPtr(ref ret, ref args) => Type::FuncPtr(
                Box::new(ret.specialize(mappings)),
                args.iter().map(|x| x.specialize(mappings)).collect(),
            ),
        }
    }

    pub fn add_dependencies_ignoring_generics(
        &self,
        generic_params: &GenericParams,
        library: &Library,
        out: &mut Dependencies,
    ) {
        match self {
            &Type::ConstPtr(ref ty) => {
                ty.add_dependencies_ignoring_generics(generic_params, library, out);
            }
            &Type::Ptr(ref ty) => {
                ty.add_dependencies_ignoring_generics(generic_params, library, out);
            }
            &Type::Path(ref path) => {
                for generic_value in &path.generics {
                    generic_value.add_dependencies_ignoring_generics(generic_params, library, out);
                }
                if !generic_params.contains(&path.name) {
                    if let Some(items) = library.get_items(&path.name) {
                        if !out.items.contains(&path.name) {
                            out.items.insert(path.name.clone());

                            for item in &items {
                                item.deref().add_dependencies(library, out);
                            }
                            for item in items {
                                out.order.push(item);
                            }
                        }
                    } else {
                        warn!(
                            "Can't find {}. This usually means that this type was incompatible or \
                             not found.",
                            path.name
                        );
                    }
                }
            }
            &Type::Primitive(_) => {}
            &Type::Array(ref ty, _) => {
                ty.add_dependencies_ignoring_generics(generic_params, library, out);
            }
            &Type::FuncPtr(ref ret, ref args) => {
                ret.add_dependencies_ignoring_generics(generic_params, library, out);
                for arg in args {
                    arg.add_dependencies_ignoring_generics(generic_params, library, out);
                }
            }
        }
    }

    pub fn add_dependencies(&self, library: &Library, out: &mut Dependencies) {
        self.add_dependencies_ignoring_generics(&GenericParams::default(), library, out)
    }

    pub fn add_monomorphs(&self, library: &Library, out: &mut Monomorphs) {
        match self {
            &Type::ConstPtr(ref ty) => {
                ty.add_monomorphs(library, out);
            }
            &Type::Ptr(ref ty) => {
                ty.add_monomorphs(library, out);
            }
            &Type::Path(ref path) => {
                if path.generics.len() == 0 || out.contains(&path) {
                    return;
                }

                if let Some(items) = library.get_items(&path.name) {
                    for item in items {
                        item.deref()
                            .instantiate_monomorph(&path.generics, library, out);
                    }
                }
            }
            &Type::Primitive(_) => {}
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

    pub fn mangle_paths(&mut self, monomorphs: &Monomorphs) {
        match self {
            &mut Type::ConstPtr(ref mut ty) => {
                ty.mangle_paths(monomorphs);
            }
            &mut Type::Ptr(ref mut ty) => {
                ty.mangle_paths(monomorphs);
            }
            &mut Type::Path(ref mut path) => {
                if path.generics.len() == 0 {
                    return;
                }

                if let Some(mangled) = monomorphs.mangle_path(path) {
                    path.name = mangled.clone();
                    path.generics = Vec::new();
                } else {
                    error!(
                        "Cannot find a mangling for generic path {:?}. This usually means that a \
                         type referenced by this generic was incompatible or not found.",
                        path
                    );
                }
            }
            &mut Type::Primitive(_) => {}
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

impl Source for String {
    fn write<F: Write>(&self, _config: &Config, out: &mut SourceWriter<F>) {
        write!(out, "{}", self);
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
            &syn::FunctionRetTy::Ty(ref t) => if let Some(x) = Type::load(t)? {
                Ok(x)
            } else {
                Ok(Type::Primitive(PrimitiveType::Void))
            },
        }
    }
}
