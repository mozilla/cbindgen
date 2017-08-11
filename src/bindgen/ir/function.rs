/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use syn;

use bindgen::annotation::*;
use bindgen::cdecl;
use bindgen::config::{Config, Layout};
use bindgen::ir::*;
use bindgen::library::*;
use bindgen::rename::*;
use bindgen::utilities::*;
use bindgen::writer::*;
use bindgen::mangle;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FunctionWriteMode {
    Global,
    MemberFunction,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub annotations: AnnotationSet,
    pub ret: Type,
    pub args: Vec<(String, Type)>,
    pub extern_decl: bool,
    pub documentation: Documentation,
}

impl Function {
    pub fn load(name: String,
                annotations: AnnotationSet,
                decl: &syn::FnDecl,
                extern_decl: bool,
                doc: String) -> Result<Function, String>
    {
        let args = decl.inputs.iter()
                              .try_skip_map(|x| x.as_ident_and_type())?;
        let ret = decl.output.as_type()?;

        Ok(Function {
            name: name,
            annotations: annotations,
            ret: ret,
            args: args,
            extern_decl: extern_decl,
            documentation: Documentation::load(doc),
        })
    }

    pub fn add_member_function(&self, out: &mut MemberFunctions) {
        if let Some(&(_, ref ty)) = self.args.get(0) {
            match *ty {
                Type::ConstPtr(ref t) | Type::Ptr(ref t) => {
                    let t = match **t {
                        Type::Path(ref p, ref g) => {
                            Type::Path(mangle::mangle_path(p, g), Vec::new())
                        }
                        _ => return
                    };
                    out.entry(t)
                        .or_insert_with(Vec::new)
                        .push(self.clone())
                }
                _ => {}
            }
        }
    }

    pub fn add_deps(&self, library: &Library, out: &mut DependencyList) {
        self.ret.add_deps(library, out);
        for &(_, ref ty) in &self.args {
            ty.add_deps(library, out);
        }
    }

    pub fn add_monomorphs(&self, library: &Library, out: &mut Monomorphs) {
        self.ret.add_monomorphs(library, out);
        for &(_, ref ty) in &self.args {
            ty.add_monomorphs(library, out);
        }
    }

    pub fn add_specializations(&self, library: &Library, out: &mut SpecializationList) {
        self.ret.add_specializations(library, out);
        for &(_, ref ty) in &self.args {
            ty.add_specializations(library, out);
        }
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

    pub fn mangle_paths(&mut self, monomorphs: &Monomorphs) {
        self.ret.mangle_paths(monomorphs);
        for &mut (_, ref mut ty) in &mut self.args {
            ty.mangle_paths(monomorphs);
        }
    }

    pub fn write_formated<F: Write>(&self,
                                    config: &Config,
                                    out: &mut SourceWriter<F>,
                                    mode: FunctionWriteMode)
    {
        fn write_1<W: Write>(func: &Function,
                             config: &Config,
                             out: &mut SourceWriter<W>,
                             mode: FunctionWriteMode)
        {
            let prefix = config.function.prefix(&func.annotations);
            let postfix = config.function.postfix(&func.annotations);

            func.documentation.write(config, out);
            if let Some(ref prefix) = prefix {
                out.write(prefix);
                out.write(" ");
            }
            if mode == FunctionWriteMode::Global {
                cdecl::write_func(out, &func, false);
            } else {
                let f = Function {
                    args: func.args[1..].to_owned(),
                    ..func.clone()
                };
                cdecl::write_func(out, &f, false);
                if let Type::ConstPtr(_) = func.args[0].1 {
                    out.write(" const");
                }
            }
            if let Some(ref postfix) = postfix {
                out.write(" ");
                out.write(postfix);
            }
            if mode == FunctionWriteMode::Global {
                out.write(";");
            }
        }

        fn write_2<W: Write>(func: &Function,
                             config: &Config,
                             out: &mut SourceWriter<W>,
                             mode: FunctionWriteMode)
        {
            let prefix = config.function.prefix(&func.annotations);
            let postfix = config.function.postfix(&func.annotations);

            func.documentation.write(config, out);
            if let Some(ref prefix) = prefix {
                out.write(prefix);
                out.new_line();
            }
            if mode == FunctionWriteMode::Global {
                cdecl::write_func(out, &func, true);
            } else {
                let f = Function {
                    args: func.args[1..].to_owned(),
                    ..func.clone()
                };
                cdecl::write_func(out, &f, true);
                if let Type::ConstPtr(_) = func.args[0].1 {
                    out.write(" const");
                }
            }
            if let Some(ref postfix) = postfix {
                out.new_line();
                out.write(postfix);
            }
            if mode == FunctionWriteMode::Global {
                out.write(";");
            }
        };

        let option_1 = out.measure(|out| write_1(self, config, out, mode));

        if (config.function.args == Layout::Auto && option_1 <= config.line_length) ||
           config.function.args == Layout::Horizontal {
            write_1(self, config, out, mode);
        } else {
            write_2(self, config, out, mode);
        }
    }
}

impl Source for Function {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        self.write_formated(config, out, FunctionWriteMode::Global)
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
