/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;
use std::fmt;

use syn;

use bindgen::cdecl;
use bindgen::config::Config;
use bindgen::ir::{Documentation, GenericPath, Item, Path};
use bindgen::dependencies::DependencyKind;
use bindgen::library::Library;
use bindgen::utilities::IterHelpers;
use bindgen::writer::{Source, SourceWriter};
use bindgen::mangle;

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
    Path(GenericPath),
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
                let path = GenericPath::load(path)?;

                if path.name == "PhantomData" {
                    return Ok(None);
                }

                if let Some(prim) = PrimitiveType::maybe(&path.name) {
                    if path.generics.len() > 0 {
                        return Err(format!("primitive has generics"));
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

    pub fn get_root_path(&self) -> Option<Path> {
        let mut current = self;
        loop {
            match current {
                &Type::ConstPtr(ref ty) => { current = ty },
                &Type::Ptr(ref ty) => { current = ty },
                &Type::Path(ref path) => {
                    return Some(path.name.clone());
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
            &Type::Path(ref path) => {
                for &(param, value) in mappings {
                    if *path.name == *param {
                        return value.clone();
                    }
                }

                let specialized = GenericPath::new(path.name.clone(),
                                                   path.generics.iter()
                                                                .map(|x| x.specialize(mappings))
                                                                .collect());
                Type::Path(specialized)
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


    pub fn mangle_paths(&mut self) {
        match self {
            &mut Type::ConstPtr(ref mut ty) => {
                ty.mangle_paths();
            }
            &mut Type::Ptr(ref mut ty) => {
                ty.mangle_paths();
            }
            &mut Type::Path(ref mut path) => {
                path.name = path.mangle();
                path.generics = Vec::new();
            }
            &mut Type::Primitive(_) => { }
            &mut Type::Array(ref mut ty, _) => {
                ty.mangle_paths();
            }
            &mut Type::FuncPtr(ref mut ret, ref mut args) => {
                ret.mangle_paths();
                for arg in args {
                    arg.mangle_paths();
                }
            }
        }
    }

     pub fn get_items(&self,
                     library: &Library,
                     kind: DependencyKind)
                     -> Vec<(Item, DependencyKind)> {
        match *self {
            Type::ConstPtr(ref tpe) |
            Type::Ptr(ref tpe) => tpe.get_items(library, DependencyKind::Ptr),
            Type::Array(ref tpe, _) => tpe.get_items(library, DependencyKind::Normal),
            Type::Primitive(..) => Vec::new(),
            Type::FuncPtr(ref ret, ref args) => {
                let mut ret = ret.get_items(library, DependencyKind::Normal);
                for arg in args {
                    ret.extend_from_slice(&arg.get_items(library, DependencyKind::Normal));
                }
                ret
            }
            Type::Path(ref path) => {
                if let Some(value) = library.get_item(&path.name) {
                    match value {
                        Item::Struct(s) => {
                            if s.generic_params.is_empty() {
                                vec![(Item::Struct(s), kind)]
                            } else {
                                let mut ret = Vec::new();
                                let mappings = s.generic_params
                                    .iter()
                                    .zip(path.generics.iter())
                                    .collect::<Vec<_>>();

                                let specialized = super::Specialization {
                                    name: s.name.clone(),
                                    annotations: s.annotations.clone(),
                                    generic_params: s.generic_params.clone(),
                                    documentation: s.documentation.clone(),
                                    generic_values: path.generics.clone(),
                                    cfg: s.cfg.clone(),
                                };

                                let monomorph = super::Struct {
                                    name: mangle::mangle_path(&s.name, &path.generics),
                                    fields: s.fields
                                        .iter()
                                        .map(|&(ref name, ref tpe, ref doc)| (name.clone(), tpe.specialize(&mappings), doc.clone()))
                                        .collect(),
                                    generic_params: vec![],
                                    specialization: Some(specialized),
                                    ..s
                                };
                                ret.push((Item::Struct(monomorph), kind));

                                ret
                            }
                        }
                        Item::Typedef(t) => {
                            if t.generic_params.is_empty() {
                                vec![(Item::Typedef(t), kind)]
                            } else {
                                let mappings = t.generic_params
                                    .iter()
                                    .zip(path.generics.iter())
                                    .collect::<Vec<_>>();
                                let specialized = super::Specialization {
                                    name: t.name.clone(),
                                    annotations: t.annotations.clone(),
                                    generic_params: t.generic_params.clone(),
                                    documentation: t.documentation.clone(),
                                    generic_values: path.generics.clone(),
                                    cfg: t.cfg.clone(),
                                };
                                let monomorph = super::Typedef {
                                    name: mangle::mangle_path(&t.name, &path.generics),
                                    generic_params: vec![],
                                    aliased: t.aliased.specialize(&mappings),
                                    specialization: Some(specialized),
                                    ..t
                                };
                                vec![(Item::Typedef(monomorph), kind)]
                            }
                        }
                        Item::OpaqueItem(o) => {
                            if o.generic_params.is_empty() {
                                vec![(Item::OpaqueItem(o), kind)]
                            } else {
                                let monomorph = super::OpaqueItem {
                                    name: mangle::mangle_path(&o.name, &path.generics),
                                    generic_params: vec![],
                                    ..o
                                };
                                vec![(Item::OpaqueItem(monomorph), kind)]
                            }
                        }
                        i => vec![(i, kind)],
                    }
                } else {
                    Vec::new()
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
