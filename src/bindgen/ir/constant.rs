/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Write;

use syn::{self, UnOp};

use bindgen::config::{Config, Language};
use bindgen::declarationtyperesolver::DeclarationTypeResolver;
use bindgen::dependencies::Dependencies;
use bindgen::ir::{
    AnnotationSet, Cfg, ConditionWrite, Documentation, GenericParams, Item, ItemContainer, Path,
    Struct, ToCondition, Type,
};
use bindgen::library::Library;
use bindgen::writer::{Source, SourceWriter};
use bindgen::Bindings;

#[derive(Debug, Clone)]
pub enum Literal {
    Expr(String),
    Path(String),
    PostfixUnaryOp {
        op: &'static str,
        value: Box<Literal>,
    },
    BinOp {
        left: Box<Literal>,
        op: &'static str,
        right: Box<Literal>,
    },
    Struct {
        path: Path,
        export_name: String,
        fields: HashMap<String, Literal>,
    },
}

impl Literal {
    fn replace_self_with(&mut self, self_ty: &Path) {
        match *self {
            Literal::PostfixUnaryOp { .. }
            | Literal::BinOp { .. }
            | Literal::Expr(..)
            | Literal::Path(..) => {}
            Literal::Struct {
                ref mut path,
                ref mut export_name,
                ref mut fields,
            } => {
                if path.replace_self_with(self_ty) {
                    *export_name = self_ty.name().to_owned();
                }
                for (ref _name, ref mut expr) in fields {
                    expr.replace_self_with(self_ty);
                }
            }
        }
    }

    fn is_valid(&self, bindings: &Bindings) -> bool {
        match *self {
            Literal::Expr(..) => true,
            Literal::Path(..) => true,
            Literal::PostfixUnaryOp { ref value, .. } => value.is_valid(bindings),
            Literal::BinOp {
                ref left,
                ref right,
                ..
            } => left.is_valid(bindings) && right.is_valid(bindings),
            Literal::Struct { ref path, .. } => bindings.struct_exists(path),
        }
    }
}

impl Literal {
    pub fn rename_for_config(&mut self, config: &Config) {
        match self {
            Literal::Struct {
                ref mut export_name,
                fields,
                ..
            } => {
                config.export.rename(export_name);
                for (_, lit) in fields {
                    lit.rename_for_config(config);
                }
            }
            Literal::Path(ref mut name) => {
                config.export.rename(name);
            }
            Literal::PostfixUnaryOp { ref mut value, .. } => {
                value.rename_for_config(config);
            }
            Literal::BinOp {
                ref mut left,
                ref mut right,
                ..
            } => {
                left.rename_for_config(config);
                right.rename_for_config(config);
            }
            Literal::Expr(_) => {}
        }
    }

    // Translate from full blown `syn::Expr` into a simpler `Literal` type
    pub fn load(expr: &syn::Expr) -> Result<Literal, String> {
        match *expr {
            // Match binary expressions of the form `a * b`
            syn::Expr::Binary(ref bin_expr) => {
                let l = Self::load(&bin_expr.left)?;
                let r = Self::load(&bin_expr.right)?;
                let op = match bin_expr.op {
                    syn::BinOp::Add(..) => "+",
                    syn::BinOp::Sub(..) => "-",
                    syn::BinOp::Mul(..) => "*",
                    syn::BinOp::Div(..) => "/",
                    syn::BinOp::Rem(..) => "%",
                    syn::BinOp::And(..) => "&&",
                    syn::BinOp::Or(..) => "||",
                    syn::BinOp::BitXor(..) => "^",
                    syn::BinOp::BitAnd(..) => "&",
                    syn::BinOp::BitOr(..) => "|",
                    syn::BinOp::Shl(..) => "<<",
                    syn::BinOp::Shr(..) => ">>",
                    syn::BinOp::Eq(..) => "==",
                    syn::BinOp::Lt(..) => "<",
                    syn::BinOp::Le(..) => "<=",
                    syn::BinOp::Ne(..) => "!=",
                    syn::BinOp::Ge(..) => ">=",
                    syn::BinOp::Gt(..) => ">",
                    syn::BinOp::AddEq(..) => "+=",
                    syn::BinOp::SubEq(..) => "-=",
                    syn::BinOp::MulEq(..) => "*=",
                    syn::BinOp::DivEq(..) => "/=",
                    syn::BinOp::RemEq(..) => "%=",
                    syn::BinOp::BitXorEq(..) => "^=",
                    syn::BinOp::BitAndEq(..) => "&=",
                    syn::BinOp::BitOrEq(..) => "|=",
                    syn::BinOp::ShlEq(..) => ">>=",
                    syn::BinOp::ShrEq(..) => "<<=",
                };
                Ok(Literal::BinOp {
                    left: Box::new(l),
                    op,
                    right: Box::new(r),
                })
            }

            // Match literals like "one", 'a', 32 etc
            syn::Expr::Lit(syn::ExprLit { ref lit, .. }) => {
                match lit {
                    syn::Lit::Str(ref value) => {
                        Ok(Literal::Expr(format!("u8\"{}\"", value.value())))
                    }
                    syn::Lit::Byte(ref value) => Ok(Literal::Expr(format!("{}", value.value()))),
                    syn::Lit::Char(ref value) => Ok(Literal::Expr(match value.value() as u32 {
                        0..=255 => format!("'{}'", value.value().escape_default()),
                        other_code => format!(r"L'\u{:X}'", other_code),
                    })),
                    syn::Lit::Int(ref value) => {
                        Ok(Literal::Expr(value.base10_digits().to_string()))
                    }
                    syn::Lit::Float(ref value) => {
                        Ok(Literal::Expr(value.base10_digits().to_string()))
                    }
                    syn::Lit::Bool(ref value) => Ok(Literal::Expr(format!("{}", value.value))),
                    // TODO: Add support for byte string and Verbatim
                    _ => Err(format!("Unsupported literal expression. {:?}", *lit)),
                }
            }

            syn::Expr::Struct(syn::ExprStruct {
                ref path,
                ref fields,
                ..
            }) => {
                let struct_name = path.segments[0].ident.to_string();
                let mut field_map = HashMap::<String, Literal>::default();
                for field in fields {
                    let ident = match field.member {
                        syn::Member::Named(ref name) => name.to_string(),
                        syn::Member::Unnamed(ref index) => format!("_{}", index.index),
                    };
                    let key = ident.to_string();
                    let value = Literal::load(&field.expr)?;
                    field_map.insert(key, value);
                }
                Ok(Literal::Struct {
                    path: Path::new(struct_name.clone()),
                    export_name: struct_name,
                    fields: field_map,
                })
            }

            syn::Expr::Unary(syn::ExprUnary {
                ref op, ref expr, ..
            }) => match *op {
                UnOp::Neg(_) => {
                    let val = Self::load(expr)?;
                    Ok(Literal::PostfixUnaryOp {
                        op: "-",
                        value: Box::new(val),
                    })
                }
                _ => Err(format!("Unsupported Unary expression. {:?}", *op)),
            },

            // Match identifiers, like `5 << SHIFT`
            syn::Expr::Path(syn::ExprPath {
                path: syn::Path { ref segments, .. },
                ..
            }) => {
                // Handle only the simplest identifiers and error for anything else.
                if segments.len() == 1 {
                    Ok(Literal::Path(format!("{}", segments.last().unwrap().ident)))
                } else {
                    Err(format!("Unsupported path expression. {:?}", *segments))
                }
            }

            syn::Expr::Paren(syn::ExprParen { ref expr, .. }) => Self::load(expr),

            _ => Err(format!("Unsupported expression. {:?}", *expr)),
        }
    }

    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        match self {
            Literal::Expr(v) => write!(out, "{}", v),
            Literal::Path(v) => write!(out, "{}", v),
            Literal::PostfixUnaryOp { op, ref value } => {
                write!(out, "{}", op);
                value.write(config, out);
            }
            Literal::BinOp {
                ref left,
                op,
                ref right,
            } => {
                write!(out, "(");
                left.write(config, out);
                write!(out, " {} ", op);
                right.write(config, out);
                write!(out, ")");
            }
            Literal::Struct {
                export_name,
                fields,
                path,
            } => {
                if config.language == Language::C {
                    write!(out, "({})", export_name);
                } else {
                    write!(out, "{}", export_name);
                }

                write!(out, "{{ ");
                let mut is_first_field = true;
                // In C++, same order as defined is required.
                let ordered_fields = out.bindings().struct_field_names(path);
                for ordered_key in ordered_fields.iter() {
                    if let Some(ref lit) = fields.get(ordered_key) {
                        if !is_first_field {
                            write!(out, ", ");
                        } else {
                            is_first_field = false;
                        }
                        if config.language == Language::Cxx {
                            write!(out, "/* .{} = */ ", ordered_key);
                        } else {
                            write!(out, ".{} = ", ordered_key);
                        }
                        lit.write(config, out);
                    }
                }
                write!(out, " }}");
            }
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
    pub associated_to: Option<Path>,
}

fn can_handle(ty: &Type, expr: &syn::Expr) -> bool {
    if ty.is_primitive_or_ptr_primitive() {
        return true;
    }
    match *expr {
        syn::Expr::Struct(_) => true,
        _ => false,
    }
}

impl Constant {
    pub fn load(
        path: Path,
        mod_cfg: Option<&Cfg>,
        ty: &syn::Type,
        expr: &syn::Expr,
        attrs: &[syn::Attribute],
        associated_to: Option<Path>,
    ) -> Result<Constant, String> {
        let ty = Type::load(ty)?;
        let mut ty = match ty {
            Some(ty) => ty,
            None => {
                return Err("Cannot have a zero sized const definition.".to_owned());
            }
        };

        if !can_handle(&ty, expr) {
            return Err("Unhandled const definition".to_owned());
        }

        let mut lit = Literal::load(&expr)?;

        if let Some(ref associated_to) = associated_to {
            ty.replace_self_with(associated_to);
            lit.replace_self_with(associated_to);
        }

        Ok(Constant::new(
            path,
            ty,
            lit,
            Cfg::append(mod_cfg, Cfg::load(attrs)),
            AnnotationSet::load(attrs)?,
            Documentation::load(attrs),
            associated_to,
        ))
    }

    pub fn new(
        path: Path,
        ty: Type,
        value: Literal,
        cfg: Option<Cfg>,
        annotations: AnnotationSet,
        documentation: Documentation,
        associated_to: Option<Path>,
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
            associated_to,
        }
    }
}

impl Item for Constant {
    fn path(&self) -> &Path {
        &self.path
    }

    fn add_dependencies(&self, library: &Library, out: &mut Dependencies) {
        self.ty.add_dependencies(library, out);
    }

    fn export_name(&self) -> &str {
        &self.export_name
    }

    fn cfg(&self) -> Option<&Cfg> {
        self.cfg.as_ref()
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
        if self.associated_to.is_none() {
            config.export.rename(&mut self.export_name);
        }
        self.value.rename_for_config(config);
        self.ty.rename_for_config(config, &GenericParams::default()); // FIXME: should probably propagate something here
    }

    fn resolve_declaration_types(&mut self, resolver: &DeclarationTypeResolver) {
        self.ty.resolve_declaration_types(resolver);
    }
}

impl Constant {
    pub fn write_declaration<F: Write>(
        &self,
        config: &Config,
        out: &mut SourceWriter<F>,
        associated_to_struct: &Struct,
    ) {
        debug_assert!(self.associated_to.is_some());
        debug_assert!(config.language == Language::Cxx);
        debug_assert!(!associated_to_struct.is_transparent);
        debug_assert!(config.structure.associated_constants_in_body);
        debug_assert!(config.constant.allow_static_const);

        if let Type::ConstPtr(..) = self.ty {
            out.write("static ");
        } else {
            out.write("static const ");
        }
        self.ty.write(config, out);
        write!(out, " {};", self.export_name())
    }

    pub fn write<F: Write>(
        &self,
        config: &Config,
        out: &mut SourceWriter<F>,
        associated_to_struct: Option<&Struct>,
    ) {
        if let Some(assoc) = associated_to_struct {
            if assoc.is_generic() {
                return; // Not tested / implemented yet, so bail out.
            }
        }

        if !self.value.is_valid(out.bindings()) {
            return;
        }

        let associated_to_transparent = associated_to_struct.map_or(false, |s| s.is_transparent);

        let in_body = associated_to_struct.is_some()
            && config.language == Language::Cxx
            && config.structure.associated_constants_in_body
            && config.constant.allow_static_const
            && !associated_to_transparent;

        let condition = (&self.cfg).to_condition(config);
        condition.write_before(config, out);

        let name = if in_body {
            Cow::Owned(format!(
                "{}::{}",
                associated_to_struct.unwrap().export_name(),
                self.export_name(),
            ))
        } else if self.associated_to.is_none() {
            Cow::Borrowed(self.export_name())
        } else {
            let associated_name = match associated_to_struct {
                Some(s) => Cow::Borrowed(s.export_name()),
                None => {
                    let mut name = self.associated_to.as_ref().unwrap().name().to_owned();
                    config.export.rename(&mut name);
                    Cow::Owned(name)
                }
            };

            Cow::Owned(format!("{}_{}", associated_name, self.export_name()))
        };

        let value = match self.value {
            Literal::Struct {
                ref fields,
                ref path,
                ..
            } if out.bindings().struct_is_transparent(path) => &fields.iter().next().unwrap().1,
            _ => &self.value,
        };

        if config.constant.allow_static_const && config.language == Language::Cxx {
            out.write(if in_body { "inline " } else { "static " });
            if let Type::ConstPtr(..) = self.ty {
                // Nothing.
            } else {
                out.write("const ");
            }
            self.ty.write(config, out);
            write!(out, " {} = ", name);
            value.write(config, out);
            write!(out, ";");
        } else {
            write!(out, "#define {} ", name);
            value.write(config, out);
        }
        condition.write_after(config, out);
    }
}
