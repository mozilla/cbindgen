/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use syn;

use crate::bindgen::cdecl;
use crate::bindgen::config::{Config, Layout};
use crate::bindgen::declarationtyperesolver::DeclarationTypeResolver;
use crate::bindgen::dependencies::Dependencies;
use crate::bindgen::ir::{
    AnnotationSet, Cfg, ConditionWrite, Documentation, GenericPath, Path, PrimitiveType,
    ToCondition, Type,
};
use crate::bindgen::library::Library;
use crate::bindgen::monomorph::Monomorphs;
use crate::bindgen::rename::{IdentifierType, RenameRule};
use crate::bindgen::reserved;
use crate::bindgen::utilities::{find_first_some, IterHelpers};
use crate::bindgen::writer::{Source, SourceWriter};

#[derive(Debug, Clone)]
pub struct Function {
    pub path: Path,
    /// Path to the self-type of the function
    /// If the function is a method, this will contain the path of the type in the impl block
    pub self_type_path: Option<Path>,
    pub ret: Type,
    pub args: Vec<(String, Type)>,
    pub extern_decl: bool,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}

impl Function {
    pub fn load(
        path: Path,
        self_type_path: Option<&Path>,
        sig: &syn::Signature,
        extern_decl: bool,
        attrs: &[syn::Attribute],
        mod_cfg: Option<&Cfg>,
    ) -> Result<Function, String> {
        let mut args = sig.inputs.iter().try_skip_map(|x| x.as_ident_and_type())?;

        let mut ret = match sig.output {
            syn::ReturnType::Default => Type::Primitive(PrimitiveType::Void),
            syn::ReturnType::Type(_, ref ty) => {
                Type::load(ty)?.unwrap_or_else(|| Type::Primitive(PrimitiveType::Void))
            }
        };

        if let Some(self_path) = self_type_path {
            for (_, ref mut ty) in &mut args {
                ty.replace_self_with(self_path);
            }
            ret.replace_self_with(self_path);
        }

        Ok(Function {
            path,
            self_type_path: self_type_path.cloned(),
            ret,
            args,
            extern_decl,
            cfg: Cfg::append(mod_cfg, Cfg::load(attrs)),
            annotations: AnnotationSet::load(attrs)?,
            documentation: Documentation::load(attrs),
        })
    }

    pub fn swift_name(&self) -> String {
        // If the symbol name starts with the type name, separate the two components with '.'
        // so that Swift recognises the association between the method and the type
        let (ref type_prefix, ref type_name) = match self.self_type_path {
            Some(ref type_name) => {
                let type_name = type_name.to_string();
                if !self.path.name().starts_with(&type_name) {
                    return self.path.to_string();
                }
                (format!("{}.", type_name), type_name)
            }
            None => ("".to_string(), "".to_string()),
        };

        let item_name = self
            .path
            .name()
            .trim_start_matches(type_name)
            .trim_start_matches('_');

        let item_args = {
            let mut items = vec![];
            for (arg, _) in self.args.iter() {
                items.push(format!("{}:", arg.as_str()));
            }
            items.join("")
        };
        format!("{}{}({})", type_prefix, item_name, item_args)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn simplify_standard_types(&mut self) {
        self.ret.simplify_standard_types();
        for &mut (_, ref mut ty) in &mut self.args {
            ty.simplify_standard_types();
        }
    }

    pub fn add_dependencies(&self, library: &Library, out: &mut Dependencies) {
        self.ret.add_dependencies(library, out);
        for &(_, ref ty) in &self.args {
            ty.add_dependencies(library, out);
        }
    }

    pub fn add_monomorphs(&self, library: &Library, out: &mut Monomorphs) {
        self.ret.add_monomorphs(library, out);
        for &(_, ref ty) in &self.args {
            ty.add_monomorphs(library, out);
        }
    }

    pub fn mangle_paths(&mut self, monomorphs: &Monomorphs) {
        self.ret.mangle_paths(monomorphs);
        for &mut (_, ref mut ty) in &mut self.args {
            ty.mangle_paths(monomorphs);
        }
    }

    pub fn resolve_declaration_types(&mut self, resolver: &DeclarationTypeResolver) {
        self.ret.resolve_declaration_types(resolver);
        for &mut (_, ref mut ty) in &mut self.args {
            ty.resolve_declaration_types(resolver);
        }
    }

    pub fn rename_for_config(&mut self, config: &Config) {
        // Rename the types used in arguments
        let generic_params = Default::default();
        self.ret.rename_for_config(config, &generic_params);
        for &mut (_, ref mut ty) in &mut self.args {
            ty.rename_for_config(config, &generic_params);
        }

        // Apply rename rules to argument names
        let rules = [
            self.annotations.parse_atom::<RenameRule>("rename-all"),
            config.function.rename_args,
        ];

        if let Some(r) = find_first_some(&rules) {
            self.args = self
                .args
                .iter()
                .map(|x| {
                    (
                        r.apply_to_snake_case(&x.0, IdentifierType::FunctionArg),
                        x.1.clone(),
                    )
                })
                .collect()
        }

        // Escape C/C++ reserved keywords used in argument names
        for args in &mut self.args {
            reserved::escape(&mut args.0);
        }
    }
}

impl Source for Function {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        fn write_1<W: Write>(func: &Function, config: &Config, out: &mut SourceWriter<W>) {
            let prefix = config.function.prefix(&func.annotations);
            let postfix = config.function.postfix(&func.annotations);

            let condition = func.cfg.to_condition(config);
            condition.write_before(config, out);

            func.documentation.write(config, out);

            if func.extern_decl {
                out.write("extern ");
            } else {
                if let Some(ref prefix) = prefix {
                    write!(out, "{} ", prefix);
                }
                if func.annotations.must_use {
                    if let Some(ref anno) = config.function.must_use {
                        write!(out, "{} ", anno);
                    }
                }
            }
            cdecl::write_func(out, &func, false, config);

            if !func.extern_decl {
                if let Some(ref postfix) = postfix {
                    write!(out, " {}", postfix);
                }
            }

            if let Some(ref swift_name_macro) = config.function.swift_name_macro {
                write!(out, " {}({})", swift_name_macro, func.swift_name());
            }

            out.write(";");

            condition.write_after(config, out);
        }

        fn write_2<W: Write>(func: &Function, config: &Config, out: &mut SourceWriter<W>) {
            let prefix = config.function.prefix(&func.annotations);
            let postfix = config.function.postfix(&func.annotations);

            let condition = func.cfg.to_condition(config);

            condition.write_before(config, out);

            func.documentation.write(config, out);

            if func.extern_decl {
                out.write("extern ");
            } else {
                if let Some(ref prefix) = prefix {
                    write!(out, "{}", prefix);
                    out.new_line();
                }
                if func.annotations.must_use {
                    if let Some(ref anno) = config.function.must_use {
                        write!(out, "{}", anno);
                        out.new_line();
                    }
                }
            }
            cdecl::write_func(out, &func, true, config);
            if !func.extern_decl {
                if let Some(ref postfix) = postfix {
                    out.new_line();
                    write!(out, "{}", postfix);
                }
            }

            if let Some(ref swift_name_macro) = config.function.swift_name_macro {
                write!(out, " {}({})", swift_name_macro, func.swift_name());
            }

            out.write(";");

            condition.write_after(config, out);
        };

        let option_1 = out.measure(|out| write_1(self, config, out));

        if (config.function.args == Layout::Auto && option_1 <= config.line_length)
            || config.function.args == Layout::Horizontal
        {
            write_1(self, config, out);
        } else {
            write_2(self, config, out);
        }
    }
}

pub trait SynFnArgHelpers {
    fn as_ident_and_type(&self) -> Result<Option<(String, Type)>, String>;
}

fn gen_self_type(receiver: &syn::Receiver) -> Type {
    let self_ty = Type::Path(GenericPath::self_path());
    match receiver.reference {
        Some(_) => match receiver.mutability {
            Some(_) => Type::Ptr(Box::new(self_ty)),
            None => Type::ConstPtr(Box::new(self_ty)),
        },
        None => self_ty,
    }
}

impl SynFnArgHelpers for syn::FnArg {
    fn as_ident_and_type(&self) -> Result<Option<(String, Type)>, String> {
        match *self {
            syn::FnArg::Typed(syn::PatType {
                ref pat, ref ty, ..
            }) => match **pat {
                syn::Pat::Ident(syn::PatIdent { ref ident, .. }) => {
                    let ty = match Type::load(ty)? {
                        Some(x) => x,
                        None => return Ok(None),
                    };
                    Ok(Some((ident.to_string(), ty)))
                }
                _ => Err("Parameter has an unsupported type.".to_owned()),
            },
            syn::FnArg::Receiver(ref receiver) => {
                Ok(Some(("self".to_string(), gen_self_type(receiver))))
            }
        }
    }
}
