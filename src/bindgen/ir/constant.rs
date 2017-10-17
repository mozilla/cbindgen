/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;
use std::mem;

use syn;

use bindgen::config::{Config, Language};
use bindgen::ir::{AnnotationSet, Cfg, Documentation, Item, ItemContainer, Specialization, Type};
use bindgen::library::Library;
use bindgen::writer::{Source, SourceWriter};

#[derive(Debug, Clone)]
pub struct LiteralExpr(String);

impl LiteralExpr {
    pub fn load(expr: &syn::Expr) -> Result<LiteralExpr, String> {
        match &expr.node {
            &syn::ExprKind::Lit(syn::Lit::Str(ref text, ..)) => {
                Ok(LiteralExpr(format!("\"{}\"", text)))
            }
            &syn::ExprKind::Lit(syn::Lit::Byte(value)) => {
                Ok(LiteralExpr(format!("{}", value)))
            }
            &syn::ExprKind::Lit(syn::Lit::Char(value)) => {
                Ok(LiteralExpr(format!("{}", value)))
            }
            &syn::ExprKind::Lit(syn::Lit::Int(value, ref ty)) => {
                match ty {
                    &syn::IntTy::Usize |
                    &syn::IntTy::U8 |
                    &syn::IntTy::U16 |
                    &syn::IntTy::U32 |
                    &syn::IntTy::U64 |
                    &syn::IntTy::Unsuffixed => {
                        Ok(LiteralExpr(format!("{}", value)))
                    }
                    &syn::IntTy::Isize |
                    &syn::IntTy::I8 |
                    &syn::IntTy::I16 |
                    &syn::IntTy::I32 |
                    &syn::IntTy::I64 => {
                        unsafe {
                            Ok(LiteralExpr(format!("{}", mem::transmute::<u64, i64>(value))))
                        }
                    }
                }
            }
            &syn::ExprKind::Lit(syn::Lit::Float(ref value, ref _ty)) => {
                Ok(LiteralExpr(format!("{}", value)))
            }
            &syn::ExprKind::Lit(syn::Lit::Bool(value)) => {
                Ok(LiteralExpr(format!("{}", value)))
            }
            _ => Err("Unsupported literal expression.".to_owned())
        }
    }
}

#[derive(Debug, Clone)]
pub struct Constant {
    pub name: String,
    pub ty: Type,
    pub value: LiteralExpr,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}


impl Constant {
    pub fn load(name: String,
                ty: &syn::Ty,
                expr: &syn::Expr,
                attrs: &Vec<syn::Attribute>,
                mod_cfg: &Option<Cfg>) -> Result<Constant, String>
    {
        let ty = Type::load(ty)?;

        if ty.is_none() {
            return Err("Cannot have a zero sized const definition.".to_owned());
        }

        let ty = ty.unwrap();

        if !ty.is_primitive_or_ptr_primitive() {
            return Err("Cannot have a non primitive const definition.".to_owned());
        }

        Ok(Constant {
            name: name,
            ty: ty,
            value: LiteralExpr::load(expr)?,
            cfg: Cfg::append(mod_cfg, Cfg::load(attrs)),
            annotations: AnnotationSet::load(attrs)?,
            documentation: Documentation::load(attrs),
        })
    }
}

impl Item for Constant {
    fn name(&self) -> &str {
        &self.name
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

    fn specialize(&self, _library: &Library, _aliasee: &Specialization) -> Result<Box<Item>, String> {
        unreachable!();
    }
}

impl Source for Constant {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        if config.constant.allow_static_const &&
           config.language == Language::Cxx
        {
            if let Type::ConstPtr(..) = self.ty {
                out.write("static ");
            } else {
                out.write("static const ");
            }
            self.ty.write(config, out);
            out.write(&format!(" {} = {};", self.name, self.value.0))
        } else {
            out.write(&format!("#define {} {}", self.name, self.value.0))
        }
    }
}
