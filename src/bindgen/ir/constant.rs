/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::fmt;
use std::io::Write;
use std::mem;

use syn;

use bindgen::config::{Config, Language};
use bindgen::declarationtyperesolver::DeclarationTypeResolver;
use bindgen::ir::{
    AnnotationSet, Cfg, ConditionWrite, Documentation, GenericParams, Item, ItemContainer, Path,
    ToCondition, Type,
};
use bindgen::writer::{Source, SourceWriter};

#[derive(Debug, Clone)]
pub enum Literal {
    Expr(String),
    Struct {
        name: String,
        export_name: String,
        fields: Vec<(String, Literal)>,
    },
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Expr(v) => write!(f, "{}", v),
            Literal::Struct {
                name: _,
                export_name,
                fields,
            } => write!(
                f,
                "({}){{ {} }}",
                export_name,
                fields
                    .iter()
                    .map(|(key, lit)| format!(".{} = {}", key, lit))
                    .collect::<Vec<String>>()
                    .join(", "),
            ),
        }
    }
}

impl Literal {
    pub fn rename_for_config(&mut self, config: &Config) {
        match self {
            Literal::Struct {
                name: _,
                ref mut export_name,
                fields,
            } => {
                config.export.rename(export_name);
                for (_, lit) in fields {
                    lit.rename_for_config(config);
                }
            }
            Literal::Expr(_) => {}
        }
    }

    pub fn load(expr: &syn::Expr) -> Result<Literal, String> {
        match expr {
            &syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(ref value),
                ..
            }) => Ok(Literal::Expr(format!("u8\"{}\"", value.value()))),
            &syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Byte(ref value),
                ..
            }) => Ok(Literal::Expr(format!("{}", value.value()))),
            &syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Char(ref value),
                ..
            }) => Ok(Literal::Expr(format!("{}", value.value()))),
            &syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(ref value),
                ..
            }) => match value.suffix() {
                syn::IntSuffix::Usize
                | syn::IntSuffix::U8
                | syn::IntSuffix::U16
                | syn::IntSuffix::U32
                | syn::IntSuffix::U64
                | syn::IntSuffix::U128
                | syn::IntSuffix::None => Ok(Literal::Expr(format!("{}", value.value()))),
                syn::IntSuffix::Isize
                | syn::IntSuffix::I8
                | syn::IntSuffix::I16
                | syn::IntSuffix::I32
                | syn::IntSuffix::I64
                | syn::IntSuffix::I128 => unsafe {
                    Ok(Literal::Expr(format!(
                        "{}",
                        mem::transmute::<u64, i64>(value.value())
                    )))
                },
            },
            &syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Float(ref value),
                ..
            }) => Ok(Literal::Expr(format!("{}", value.value()))),
            &syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Bool(ref value),
                ..
            }) => Ok(Literal::Expr(format!("{}", value.value))),

            &syn::Expr::Struct(syn::ExprStruct {
                ref path,
                ref fields,
                ..
            }) => {
                let struct_name = path.segments[0].ident.to_string();
                let mut field_pairs: Vec<(String, Literal)> = Vec::new();
                for field in fields {
                    let ident = match field.member {
                        syn::Member::Named(ref name) => name.to_string(),
                        syn::Member::Unnamed(ref index) => format!("_{}", index.index),
                    };
                    let key = ident.to_string();
                    let value = Literal::load(&field.expr)?;
                    field_pairs.push((key, value));
                }
                Ok(Literal::Struct {
                    name: struct_name.clone(),
                    export_name: struct_name,
                    fields: field_pairs,
                })
            }
            _ => Err("Unsupported literal expression.".to_owned()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Constant {
    pub path: Path,
    pub export_name: String,
    pub ty: Type,
    pub value: Literal,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}

impl Constant {
    pub fn load(
        path: Path,
        item: &syn::ItemConst,
        mod_cfg: &Option<Cfg>,
    ) -> Result<Constant, String> {
        let ty = Type::load(&item.ty)?;

        if ty.is_none() {
            return Err("Cannot have a zero sized const definition.".to_owned());
        }

        let ty = ty.unwrap();

        if !ty.is_primitive_or_ptr_primitive()
            && match *item.expr {
                syn::Expr::Struct(_) => false,
                _ => true,
            }
        {
            return Err("Unhanded const definition".to_owned());
        }

        Ok(Constant::new(
            path,
            ty,
            Literal::load(&item.expr)?,
            Cfg::append(mod_cfg, Cfg::load(&item.attrs)),
            AnnotationSet::load(&item.attrs)?,
            Documentation::load(&item.attrs),
        ))
    }

    pub fn load_assoc(
        name: String,
        item: &syn::ImplItemConst,
        impl_ty: &syn::Type,
        mod_cfg: &Option<Cfg>,
    ) -> Result<Constant, String> {
        let ty = Type::load(&item.ty)?;

        if ty.is_none() {
            return Err("Cannot have a zero sized const definition.".to_owned());
        }
        let ty = ty.unwrap();
        if !ty.is_primitive_or_ptr_primitive()
            && match item.expr {
                syn::Expr::Struct(_) => false,
                _ => true,
            }
        {
            return Err("Unhanded const definition".to_owned());
        }

        let impl_ty = Type::load(impl_ty)?;
        if impl_ty.is_none() {
            return Err("impl has an empty type".to_owned());
        }
        let impl_ty = impl_ty.unwrap();

        let struct_path = impl_ty.get_root_path().unwrap();
        let full_name = Path::new(format!("{}_{}", struct_path, name));

        Ok(Constant::new(
            full_name,
            ty,
            Literal::load(&item.expr)?,
            Cfg::append(mod_cfg, Cfg::load(&item.attrs)),
            AnnotationSet::load(&item.attrs)?,
            Documentation::load(&item.attrs),
        ))
    }

    pub fn new(
        path: Path,
        ty: Type,
        value: Literal,
        cfg: Option<Cfg>,
        annotations: AnnotationSet,
        documentation: Documentation,
    ) -> Self {
        let export_name = path.name().to_owned();
        Self {
            path,
            export_name,
            ty,
            value,
            cfg,
            annotations,
            documentation,
        }
    }
}

impl Item for Constant {
    fn path(&self) -> &Path {
        &self.path
    }

    fn export_name(&self) -> &str {
        &self.export_name
    }

    fn cfg(&self) -> &Option<Cfg> {
        &self.cfg
    }

    fn annotations(&self) -> &AnnotationSet {
        &self.annotations
    }

    fn annotations_mut(&mut self) -> &mut AnnotationSet {
        &mut self.annotations
    }

    fn container(&self) -> ItemContainer {
        ItemContainer::Constant(self.clone())
    }

    fn rename_for_config(&mut self, config: &Config) {
        config.export.rename(&mut self.export_name);
        self.value.rename_for_config(config);
        self.ty.rename_for_config(config, &GenericParams::default()); // FIXME: should probably propagate something here
    }

    fn resolve_declaration_types(&mut self, resolver: &DeclarationTypeResolver) {
        self.ty.resolve_declaration_types(resolver);
    }
}

impl Source for Constant {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        let condition = (&self.cfg).to_condition(config);
        condition.write_before(config, out);
        if config.constant.allow_static_const && config.language == Language::Cxx {
            if let Type::ConstPtr(..) = self.ty {
                out.write("static ");
            } else {
                out.write("static const ");
            }
            self.ty.write(config, out);
            write!(out, " {} = {};", self.export_name(), self.value)
        } else {
            write!(out, "#define {} {}", self.export_name(), self.value)
        }
        condition.write_after(config, out);
    }
}
