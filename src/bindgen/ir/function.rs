/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use syn;

use bindgen::cdecl;
use bindgen::config::{Config, Layout, Language};
use bindgen::dependencies::Dependencies;
use bindgen::ir::{AnnotationSet, Cfg, CfgWrite, Documentation, SynFnRetTyHelpers, Type};
use bindgen::library::Library;
use bindgen::monomorph::Monomorphs;
use bindgen::rename::{IdentifierType, RenameRule};
use bindgen::utilities::{find_first_some, IterHelpers};
use bindgen::writer::{Source, SourceWriter};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub ret: Type,
    pub args: Vec<(String, Type)>,
    pub extern_decl: bool,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}

impl Function {
    pub fn load(
        name: String,
        decl: &syn::FnDecl,
        extern_decl: bool,
        attrs: &Vec<syn::Attribute>,
        mod_cfg: &Option<Cfg>,
    ) -> Result<Function, String> {
        let args = decl.inputs.iter().try_skip_map(|x| x.as_ident_and_type())?;
        let ret = decl.output.as_type()?;

        Ok(Function {
            name: name,
            ret: ret,
            args: args,
            extern_decl: extern_decl,
            cfg: Cfg::append(mod_cfg, Cfg::load(attrs)),
            annotations: AnnotationSet::load(attrs)?,
            documentation: Documentation::load(attrs),
        })
    }

    pub fn simplify_option_to_ptr(&mut self) {
        self.ret.simplify_option_to_ptr();
        for &mut (_, ref mut ty) in &mut self.args {
            ty.simplify_option_to_ptr();
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

    pub fn rename_for_config(&mut self, config: &Config) {
        self.ret.rename_for_config(config);
        for &mut (_, ref mut ty) in &mut self.args {
            ty.rename_for_config(config);
        }

        let rules = [
            self.annotations.parse_atom::<RenameRule>("rename-all"),
            config.function.rename_args,
        ];

        if let Some(r) = find_first_some(&rules) {
            self.args = self.args
                .iter()
                .map(|x| {
                    (
                        r.apply_to_snake_case(&x.0, IdentifierType::FunctionArg),
                        x.1.clone(),
                    )
                })
                .collect()
        }
    }
}

impl Source for Function {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        fn write_1<W: Write>(func: &Function, config: &Config, out: &mut SourceWriter<W>) {
            let void_prototype = config.language == Language::C;
            let prefix = config.function.prefix(&func.annotations);
            let postfix = config.function.postfix(&func.annotations);

            func.cfg.write_before(config, out);

            func.documentation.write(config, out);

            if func.extern_decl {
                out.write("extern ");
            } else {
                if let Some(ref prefix) = prefix {
                    write!(out, "{}", prefix);
                    out.write(" ");
                }
            }
            cdecl::write_func(out, &func, false, void_prototype);
            if !func.extern_decl {
                if let Some(ref postfix) = postfix {
                    out.write(" ");
                    write!(out, "{}", postfix);
                }
            }
            out.write(";");

            func.cfg.write_after(config, out);
        }

        fn write_2<W: Write>(func: &Function, config: &Config, out: &mut SourceWriter<W>) {
            let void_prototype = config.language == Language::C;
            let prefix = config.function.prefix(&func.annotations);
            let postfix = config.function.postfix(&func.annotations);

            func.cfg.write_before(config, out);

            func.documentation.write(config, out);

            if func.extern_decl {
                out.write("extern ");
            } else {
                if let Some(ref prefix) = prefix {
                    write!(out, "{}", prefix);
                    out.new_line();
                }
            }
            cdecl::write_func(out, &func, true, void_prototype);
            if !func.extern_decl {
                if let Some(ref postfix) = postfix {
                    out.new_line();
                    write!(out, "{}", postfix);
                }
            }
            out.write(";");

            func.cfg.write_after(config, out);
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

impl SynFnArgHelpers for syn::FnArg {
    fn as_ident_and_type(&self) -> Result<Option<(String, Type)>, String> {
        match self {
            &syn::FnArg::Captured(syn::Pat::Ident(_, ref ident, _), ref ty) => {
                if let Some(x) = Type::load(ty)? {
                    Ok(Some((ident.to_string(), x)))
                } else {
                    Ok(None)
                }
            }
            _ => Err("Parameter has an unsupported type.".to_owned()),
        }
    }
}
