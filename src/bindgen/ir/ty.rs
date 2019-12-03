/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::fmt;
use std::io::Write;

use syn;

use bindgen::cdecl;
use bindgen::config::Config;
use bindgen::declarationtyperesolver::DeclarationTypeResolver;
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
    SChar,
    UChar,
    Char32,
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
    SSizeT,
    PtrDiffT,
    VaList,
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
            "char" => Some(PrimitiveType::Char32),
            "usize" | "uintptr_t" => Some(PrimitiveType::USize),
            "u8" | "uint8_t" => Some(PrimitiveType::UInt8),
            "u16" | "uint16_t" => Some(PrimitiveType::UInt16),
            "u32" | "uint32_t" => Some(PrimitiveType::UInt32),
            "u64" | "uint64_t" => Some(PrimitiveType::UInt64),
            "isize" | "intptr_t" => Some(PrimitiveType::ISize),
            "i8" | "int8_t" => Some(PrimitiveType::Int8),
            "i16" | "int16_t" => Some(PrimitiveType::Int16),
            "i32" | "int32_t" => Some(PrimitiveType::Int32),
            "i64" | "int64_t" => Some(PrimitiveType::Int64),
            "f32" => Some(PrimitiveType::Float),
            "f64" => Some(PrimitiveType::Double),
            "size_t" => Some(PrimitiveType::SizeT),
            "ssize_t" => Some(PrimitiveType::SSizeT),
            "ptrdiff_t" => Some(PrimitiveType::PtrDiffT),
            "VaList" => Some(PrimitiveType::VaList),
            _ => None,
        }
    }

    pub fn to_repr_rust(&self) -> &'static str {
        match *self {
            PrimitiveType::Void => "c_void",
            PrimitiveType::Char => "c_char",
            PrimitiveType::SChar => "c_schar",
            PrimitiveType::UChar => "c_uchar",
            PrimitiveType::Char32 => "char",
            PrimitiveType::Short => "c_short",
            PrimitiveType::Int => "c_int",
            PrimitiveType::Long => "c_long",
            PrimitiveType::LongLong => "c_longlong",
            PrimitiveType::UShort => "c_ushort",
            PrimitiveType::UInt => "c_uint",
            PrimitiveType::ULong => "c_ulong",
            PrimitiveType::ULongLong => "c_ulonglong",
            PrimitiveType::Bool => "bool",
            PrimitiveType::USize => "usize",
            PrimitiveType::UInt8 => "u8",
            PrimitiveType::UInt16 => "u16",
            PrimitiveType::UInt32 => "u32",
            PrimitiveType::UInt64 => "u64",
            PrimitiveType::ISize => "isize",
            PrimitiveType::Int8 => "i8",
            PrimitiveType::Int16 => "i16",
            PrimitiveType::Int32 => "i32",
            PrimitiveType::Int64 => "i64",
            PrimitiveType::Float => "f32",
            PrimitiveType::Double => "f64",
            PrimitiveType::SizeT => "size_t",
            PrimitiveType::SSizeT => "ssize_t",
            PrimitiveType::PtrDiffT => "ptrdiff_t",
            PrimitiveType::VaList => "va_list",
        }
    }

    pub fn to_repr_c(&self) -> &'static str {
        match *self {
            PrimitiveType::Void => "void",
            PrimitiveType::Bool => "bool",
            PrimitiveType::Char => "char",
            PrimitiveType::SChar => "signed char",
            PrimitiveType::UChar => "unsigned char",
            // NOTE: It'd be nice to use a char32_t, but:
            //
            //  * uchar.h is not present on mac (see #423).
            //
            //  * char32_t isn't required to be compatible with Rust's char, as
            //    the C++ spec only requires it to be the same size as
            //    uint_least32_t, which is _not_ guaranteed to be 4-bytes.
            //
            PrimitiveType::Char32 => "uint32_t",
            PrimitiveType::Short => "short",
            PrimitiveType::Int => "int",
            PrimitiveType::Long => "long",
            PrimitiveType::LongLong => "long long",
            PrimitiveType::UShort => "unsigned short",
            PrimitiveType::UInt => "unsigned int",
            PrimitiveType::ULong => "unsigned long",
            PrimitiveType::ULongLong => "unsigned long long",
            PrimitiveType::USize => "uintptr_t",
            PrimitiveType::UInt8 => "uint8_t",
            PrimitiveType::UInt16 => "uint16_t",
            PrimitiveType::UInt32 => "uint32_t",
            PrimitiveType::UInt64 => "uint64_t",
            PrimitiveType::ISize => "intptr_t",
            PrimitiveType::Int8 => "int8_t",
            PrimitiveType::Int16 => "int16_t",
            PrimitiveType::Int32 => "int32_t",
            PrimitiveType::Int64 => "int64_t",
            PrimitiveType::Float => "float",
            PrimitiveType::Double => "double",
            PrimitiveType::SizeT => "size_t",
            PrimitiveType::SSizeT => "ssize_t",
            PrimitiveType::PtrDiffT => "ptrdiff_t",
            PrimitiveType::VaList => "va_list",
        }
    }

    fn can_cmp_order(&self) -> bool {
        match *self {
            PrimitiveType::Bool => false,
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

// The `U` part of `[T; U]`
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ArrayLength {
    Name(String),
    Value(String),
}

impl ArrayLength {
    pub fn as_str(&self) -> &str {
        match self {
            ArrayLength::Name(ref string) | ArrayLength::Value(ref string) => string,
        }
    }

    fn rename_for_config(&mut self, config: &Config) {
        if let ArrayLength::Name(ref mut name) = self {
            config.export.rename(name);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Type {
    ConstPtr(Box<Type>),
    Ptr(Box<Type>),
    Ref(Box<Type>),
    MutRef(Box<Type>),
    Path(GenericPath),
    Primitive(PrimitiveType),
    Array(Box<Type>, ArrayLength),
    FuncPtr(Box<Type>, Vec<(Option<String>, Type)>),
}

impl Type {
    pub fn load(ty: &syn::Type) -> Result<Option<Type>, String> {
        let converted = match *ty {
            syn::Type::Reference(ref reference) => {
                let converted = Type::load(&reference.elem)?;

                let converted = match converted {
                    Some(converted) => converted,
                    None => {
                        return Err("Cannot have a pointer to a zero sized type. If you are \
                                    trying to represent `void*` use `c_void*`."
                            .to_owned());
                    }
                };

                match reference.mutability {
                    Some(_) => Type::Ptr(Box::new(converted)),
                    None => Type::ConstPtr(Box::new(converted)),
                }
            }
            syn::Type::Ptr(ref pointer) => {
                let converted = Type::load(&pointer.elem)?;

                let converted = match converted {
                    Some(converted) => converted,
                    None => {
                        return Err("Cannot have a pointer to a zero sized type. If you are \
                                    trying to represent `void*` use `c_void*`."
                            .to_owned());
                    }
                };

                match pointer.mutability {
                    Some(_) => Type::Ptr(Box::new(converted)),
                    None => Type::ConstPtr(Box::new(converted)),
                }
            }
            syn::Type::Path(ref path) => {
                let generic_path = GenericPath::load(&path.path)?;

                if generic_path.name() == "PhantomData" {
                    return Ok(None);
                }

                if let Some(prim) = PrimitiveType::maybe(generic_path.name()) {
                    if !generic_path.generics().is_empty() {
                        return Err("Primitive has generics.".to_owned());
                    }
                    Type::Primitive(prim)
                } else {
                    Type::Path(generic_path)
                }
            }
            syn::Type::Array(syn::TypeArray {
                ref elem,
                len: syn::Expr::Path(ref path),
                ..
            }) => {
                let converted = Type::load(elem)?;

                let converted = match converted {
                    Some(converted) => converted,
                    None => return Err("Cannot have an array of zero sized types.".to_owned()),
                };
                let generic_path = GenericPath::load(&path.path)?;
                let len = ArrayLength::Name(generic_path.export_name().to_owned());
                Type::Array(Box::new(converted), len)
            }
            syn::Type::Array(syn::TypeArray {
                ref elem,
                len:
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Int(ref len),
                        ..
                    }),
                ..
            }) => {
                let converted = Type::load(elem)?;

                let converted = match converted {
                    Some(converted) => converted,
                    None => return Err("Cannot have an array of zero sized types.".to_owned()),
                };

                let len = ArrayLength::Value(len.base10_digits().to_string());
                // panic!("panic -> value: {:?}", len);
                Type::Array(Box::new(converted), len)
            }
            syn::Type::BareFn(ref function) => {
                let mut wildcard_counter = 0;
                let args = function.inputs.iter().try_skip_map(|x| {
                    Type::load(&x.ty).map(|opt_ty| {
                        opt_ty.map(|ty| {
                            (
                                x.name.as_ref().map(|(ref ident, _)| {
                                    if ident == "_" {
                                        wildcard_counter += 1;
                                        if wildcard_counter == 1 {
                                            "_".to_owned()
                                        } else {
                                            format!("_{}", wildcard_counter - 1)
                                        }
                                    } else {
                                        ident.to_string()
                                    }
                                }),
                                ty,
                            )
                        })
                    })
                })?;
                let ret = match function.output {
                    syn::ReturnType::Default => Type::Primitive(PrimitiveType::Void),
                    syn::ReturnType::Type(_, ref ty) => {
                        if let Some(x) = Type::load(ty)? {
                            x
                        } else {
                            Type::Primitive(PrimitiveType::Void)
                        }
                    }
                };

                Type::FuncPtr(Box::new(ret), args)
            }
            syn::Type::Tuple(ref tuple) => {
                if tuple.elems.is_empty() {
                    return Ok(None);
                }
                return Err("Tuples are not supported types.".to_owned());
            }
            _ => return Err(format!("Unsupported type: {:?}", ty)),
        };

        Ok(Some(converted))
    }

    pub fn is_primitive_or_ptr_primitive(&self) -> bool {
        match *self {
            Type::Primitive(..) => true,
            Type::ConstPtr(ref x) => match x.as_ref() {
                Type::Primitive(..) => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn is_repr_ptr(&self) -> bool {
        match *self {
            Type::Ptr(..) => true,
            Type::ConstPtr(..) => true,
            Type::FuncPtr(..) => true,
            _ => false,
        }
    }

    fn simplified_type(&self) -> Option<Self> {
        let path = match *self {
            Type::Path(ref p) => p,
            _ => return None,
        };

        if path.generics().len() != 1 {
            return None;
        }

        let mut generic = path.generics()[0].clone();
        generic.simplify_standard_types();

        match path.name() {
            // FIXME(#223): This is not quite correct.
            "Option" if generic.is_repr_ptr() => Some(generic),
            "NonNull" => Some(Type::Ptr(Box::new(generic))),
            _ => None,
        }
    }

    pub fn simplify_standard_types(&mut self) {
        if let Some(ty) = self.simplified_type() {
            *self = ty;
        }
    }

    pub fn replace_self_with(&mut self, self_ty: &Path) {
        match *self {
            Type::Array(ref mut ty, ..)
            | Type::MutRef(ref mut ty)
            | Type::Ref(ref mut ty)
            | Type::Ptr(ref mut ty)
            | Type::ConstPtr(ref mut ty) => ty.replace_self_with(self_ty),
            Type::Path(ref mut generic_path) => {
                generic_path.replace_self_with(self_ty);
            }
            Type::Primitive(..) => {}
            Type::FuncPtr(ref mut ret, ref mut args) => {
                ret.replace_self_with(self_ty);
                for arg in args {
                    arg.1.replace_self_with(self_ty);
                }
            }
        }
    }

    pub fn get_root_path(&self) -> Option<Path> {
        let mut current = self;
        loop {
            match *current {
                Type::ConstPtr(ref ty) => current = ty,
                Type::Ptr(ref ty) => current = ty,
                Type::Ref(ref ty) => current = ty,
                Type::MutRef(ref ty) => current = ty,
                Type::Path(ref generic) => {
                    return Some(generic.path().clone());
                }
                Type::Primitive(..) => {
                    return None;
                }
                Type::Array(..) => {
                    return None;
                }
                Type::FuncPtr(..) => {
                    return None;
                }
            };
        }
    }

    pub fn specialize(&self, mappings: &[(&Path, &Type)]) -> Type {
        match *self {
            Type::ConstPtr(ref ty) => Type::ConstPtr(Box::new(ty.specialize(mappings))),
            Type::Ptr(ref ty) => Type::Ptr(Box::new(ty.specialize(mappings))),
            Type::Ref(ref ty) => Type::Ref(Box::new(ty.specialize(mappings))),
            Type::MutRef(ref ty) => Type::MutRef(Box::new(ty.specialize(mappings))),
            Type::Path(ref generic_path) => {
                for &(param, value) in mappings {
                    if generic_path.path() == param {
                        return value.clone();
                    }
                }

                let specialized = GenericPath::new(
                    generic_path.path().clone(),
                    generic_path
                        .generics()
                        .iter()
                        .map(|x| x.specialize(mappings))
                        .collect(),
                );
                Type::Path(specialized)
            }
            Type::Primitive(ref primitive) => Type::Primitive(primitive.clone()),
            Type::Array(ref ty, ref constant) => {
                Type::Array(Box::new(ty.specialize(mappings)), constant.clone())
            }
            Type::FuncPtr(ref ret, ref args) => Type::FuncPtr(
                Box::new(ret.specialize(mappings)),
                args.iter()
                    .cloned()
                    .map(|(name, ty)| (name, ty.specialize(mappings)))
                    .collect(),
            ),
        }
    }

    pub fn add_dependencies_ignoring_generics(
        &self,
        generic_params: &GenericParams,
        library: &Library,
        out: &mut Dependencies,
    ) {
        match *self {
            Type::ConstPtr(ref ty) => {
                ty.add_dependencies_ignoring_generics(generic_params, library, out);
            }
            Type::Ptr(ref ty) => {
                ty.add_dependencies_ignoring_generics(generic_params, library, out);
            }
            Type::Ref(ref ty) | Type::MutRef(ref ty) => {
                ty.add_dependencies_ignoring_generics(generic_params, library, out);
            }
            Type::Path(ref generic) => {
                for generic_value in generic.generics() {
                    generic_value.add_dependencies_ignoring_generics(generic_params, library, out);
                }
                let path = generic.path();
                if !generic_params.contains(path) {
                    if let Some(items) = library.get_items(path) {
                        if !out.items.contains(path) {
                            out.items.insert(path.clone());

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
                            path
                        );
                    }
                }
            }
            Type::Primitive(_) => {}
            Type::Array(ref ty, _) => {
                ty.add_dependencies_ignoring_generics(generic_params, library, out);
            }
            Type::FuncPtr(ref ret, ref args) => {
                ret.add_dependencies_ignoring_generics(generic_params, library, out);
                for (_, ref arg) in args {
                    arg.add_dependencies_ignoring_generics(generic_params, library, out);
                }
            }
        }
    }

    pub fn add_dependencies(&self, library: &Library, out: &mut Dependencies) {
        self.add_dependencies_ignoring_generics(&GenericParams::default(), library, out)
    }

    pub fn add_monomorphs(&self, library: &Library, out: &mut Monomorphs) {
        match *self {
            Type::ConstPtr(ref ty) => {
                ty.add_monomorphs(library, out);
            }
            Type::Ptr(ref ty) => {
                ty.add_monomorphs(library, out);
            }
            Type::Ref(ref ty) | Type::MutRef(ref ty) => {
                ty.add_monomorphs(library, out);
            }
            Type::Path(ref generic) => {
                if generic.generics().is_empty() || out.contains(&generic) {
                    return;
                }
                let path = generic.path();
                if let Some(items) = library.get_items(path) {
                    for item in items {
                        item.deref()
                            .instantiate_monomorph(generic.generics(), library, out);
                    }
                }
            }
            Type::Primitive(_) => {}
            Type::Array(ref ty, _) => {
                ty.add_monomorphs(library, out);
            }
            Type::FuncPtr(ref ret, ref args) => {
                ret.add_monomorphs(library, out);
                for (_, ref arg) in args {
                    arg.add_monomorphs(library, out);
                }
            }
        }
    }

    pub fn rename_for_config(&mut self, config: &Config, generic_params: &GenericParams) {
        match *self {
            Type::ConstPtr(ref mut ty) => {
                ty.rename_for_config(config, generic_params);
            }
            Type::Ptr(ref mut ty) => {
                ty.rename_for_config(config, generic_params);
            }
            Type::Ref(ref mut ty) | Type::MutRef(ref mut ty) => {
                ty.rename_for_config(config, generic_params);
            }
            Type::Path(ref mut ty) => {
                ty.rename_for_config(config, generic_params);
            }
            Type::Primitive(_) => {}
            Type::Array(ref mut ty, ref mut len) => {
                ty.rename_for_config(config, generic_params);
                len.rename_for_config(config);
            }
            Type::FuncPtr(ref mut ret, ref mut args) => {
                ret.rename_for_config(config, generic_params);
                for (_, arg) in args {
                    arg.rename_for_config(config, generic_params);
                }
            }
        }
    }

    pub fn resolve_declaration_types(&mut self, resolver: &DeclarationTypeResolver) {
        match *self {
            Type::ConstPtr(ref mut ty) => {
                ty.resolve_declaration_types(resolver);
            }
            Type::Ptr(ref mut ty) => {
                ty.resolve_declaration_types(resolver);
            }
            Type::Ref(ref mut ty) | Type::MutRef(ref mut ty) => {
                ty.resolve_declaration_types(resolver);
            }
            Type::Path(ref mut generic_path) => {
                generic_path.resolve_declaration_types(resolver);
            }
            Type::Primitive(_) => {}
            Type::Array(ref mut ty, _) => {
                ty.resolve_declaration_types(resolver);
            }
            Type::FuncPtr(ref mut ret, ref mut args) => {
                ret.resolve_declaration_types(resolver);
                for (_, ref mut arg) in args {
                    arg.resolve_declaration_types(resolver);
                }
            }
        }
    }

    pub fn mangle_paths(&mut self, monomorphs: &Monomorphs) {
        match *self {
            Type::ConstPtr(ref mut ty) => {
                ty.mangle_paths(monomorphs);
            }
            Type::Ptr(ref mut ty) => {
                ty.mangle_paths(monomorphs);
            }
            Type::Ref(ref mut ty) | Type::MutRef(ref mut ty) => {
                ty.mangle_paths(monomorphs);
            }
            Type::Path(ref mut generic_path) => {
                if generic_path.generics().is_empty() {
                    return;
                }

                if let Some(mangled_path) = monomorphs.mangle_path(&generic_path) {
                    *generic_path = GenericPath::new(mangled_path.clone(), vec![]);
                } else {
                    error!(
                        "Cannot find a mangling for generic path {:?}. This usually means that a \
                         type referenced by this generic was incompatible or not found.",
                        generic_path
                    );
                }
            }
            Type::Primitive(_) => {}
            Type::Array(ref mut ty, _) => {
                ty.mangle_paths(monomorphs);
            }
            Type::FuncPtr(ref mut ret, ref mut args) => {
                ret.mangle_paths(monomorphs);
                for (_, ref mut arg) in args {
                    arg.mangle_paths(monomorphs);
                }
            }
        }
    }

    pub fn can_cmp_order(&self) -> bool {
        match *self {
            Type::ConstPtr(..) => true,
            Type::Ptr(..) => true,
            Type::Ref(..) | Type::MutRef(..) => false,
            Type::Path(..) => true,
            Type::Primitive(ref p) => p.can_cmp_order(),
            Type::Array(..) => false,
            Type::FuncPtr(..) => false,
        }
    }

    pub fn can_cmp_eq(&self) -> bool {
        match *self {
            Type::ConstPtr(..) => true,
            Type::Ptr(..) => true,
            Type::Ref(..) | Type::MutRef(..) => false,
            Type::Path(..) => true,
            Type::Primitive(ref p) => p.can_cmp_eq(),
            Type::Array(..) => false,
            Type::FuncPtr(..) => true,
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
