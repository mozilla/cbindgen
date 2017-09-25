/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use syn;

use bindgen::cdecl;
use bindgen::config::{Config, Layout};
use bindgen::ir::{AnnotationSet, Cfg, CfgWrite, Documentation, SynFnRetTyHelpers, Type, Item};
use bindgen::library::Library;
use bindgen::rename::{IdentifierType, RenameRule};
use bindgen::utilities::{find_first_some, IterHelpers};
use bindgen::writer::{Source, SourceWriter};
use bindgen::dependencies::DependencyKind;
use bindgen::config::Language;

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
    pub fn load(name: String,
                decl: &syn::FnDecl,
                extern_decl: bool,
                attrs: &Vec<syn::Attribute>,
                mod_cfg: &Option<Cfg>) -> Result<Function, String>
    {
        let args = decl.inputs.iter()
                              .try_skip_map(|x| x.as_ident_and_type())?;
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

    pub fn mangle_paths(&mut self) {
        self.ret.mangle_paths();
        for &mut (_, ref mut ty) in &mut self.args {
            ty.mangle_paths();
        }
    }

    pub fn get_deps(&self, library: &Library) -> Vec<(Item, DependencyKind)> {
        let mut ret = self.ret.get_items(library, DependencyKind::Normal);
        for &(_, ref arg) in &self.args {
            ret.extend_from_slice(&arg.get_items(library, DependencyKind::Normal));
        }
        ret
    }
}

impl Source for Function {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        fn write_1<W: Write>(func: &Function, config: &Config, out: &mut SourceWriter<W>) {
            let prefix = config.function.prefix(&func.annotations);
            let postfix = config.function.postfix(&func.annotations);

            func.cfg.write_before(config, out);

            func.documentation.write(config, out);

            if func.extern_decl {
                out.write("extern ");
            } else {
                if config.language == Language::Cxx {
                    out.write("extern \"C\" ");
                }
                if let Some(ref prefix) = prefix {
                    out.write(prefix);
                    out.write(" ");
                }
            }
            cdecl::write_func(out, &func, false);
            if !func.extern_decl {
                if let Some(ref postfix) = postfix {
                    out.write(" ");
                    out.write(postfix);
                }
            }
            out.write(";");

            func.cfg.write_after(config, out);
        }

        fn write_2<W: Write>(func: &Function, config: &Config, out: &mut SourceWriter<W>) {
            let prefix = config.function.prefix(&func.annotations);
            let postfix = config.function.postfix(&func.annotations);

            func.cfg.write_before(config, out);

            func.documentation.write(config, out);

            if func.extern_decl {
                out.write("extern ");
            } else {
                if config.language == Language::Cxx {
                    out.write("extern \"C\" ");
                }
                if let Some(ref prefix) = prefix {
                    out.write(prefix);
                    out.new_line();
                }
            }
            cdecl::write_func(out, &func, true);
            if !func.extern_decl {
                if let Some(ref postfix) = postfix {
                    out.new_line();
                    out.write(postfix);
                }
            }
            out.write(";");

            func.cfg.write_after(config, out);
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
            _ => Err(format!("parameter has unexpected type")),
        }
    }
}
