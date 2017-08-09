/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;
use std::io::Write;

use syn;

use bindgen::annotation::*;
use bindgen::config::Config;
use bindgen::ir::*;
use bindgen::library::*;
use bindgen::monomorph::Monomorphs;
use bindgen::writer::*;

/// A type alias that generates a copy of its aliasee with a new name. If the type
/// alias has generic values, it specializes its aliasee. This is useful for
/// presenting an interface that includes generic types without mangling.
#[derive(Debug, Clone)]
pub struct Specialization {
    pub name: String,
    pub annotations: AnnotationSet,
    pub aliased: GenericPath,
    pub generic_params: Vec<String>,
    pub documentation: Documentation,
}

impl Specialization {
    pub fn load(name: String,
                annotations: AnnotationSet,
                generics: &syn::Generics,
                ty: &syn::Ty,
                doc: String) -> Result<Specialization, String>
    {
        match ty {
            &syn::Ty::Path(ref _q, ref p) => {
                let generic_params = generics.ty_params.iter()
                                                       .map(|x| x.ident.to_string())
                                                       .collect::<Vec<_>>();

                let path = GenericPath::load(p)?;

                if PrimitiveType::maybe(&path.name).is_some() {
                    return Err(format!("can't specialize a primitive"));
                }

                Ok(Specialization {
                    name: name,
                    annotations: annotations,
                    aliased: path,
                    generic_params: generic_params,
                    documentation: Documentation::load(doc),
                })
            }
            _ => {
                Err(format!("not a path"))
            }
        }
    }

    pub fn specialize(&self, library: &Library) -> Result<Option<Item>, String> {
        match library.resolve_path(&self.aliased.name) {
            Some(aliased) => {
                match aliased {
                    Item::OpaqueItem(ref aliased) => {
                        if self.aliased.generics.len() !=
                           aliased.generic_params.len() {
                            return Err(format!("incomplete specialization"));
                        }

                        Ok(Some(Item::OpaqueItem(OpaqueItem {
                            name: self.name.clone(),
                            generic_params: self.generic_params.clone(),
                            annotations: self.annotations.clone(),
                            documentation: self.documentation.clone(),
                        })))
                    }
                    Item::Struct(ref aliased) => {
                        if self.aliased.generics.len() !=
                           aliased.generic_params.len() {
                            return Err(format!("incomplete specialization"));
                        }

                        let mappings = aliased.generic_params.iter()
                                                             .zip(self.aliased.generics.iter())
                                                             .collect::<Vec<_>>();

                        Ok(Some(Item::Struct(Struct {
                            name: self.name.clone(),
                            annotations: self.annotations.clone(),
                            fields: aliased.fields.iter()
                                                  .map(|x| (x.0.clone(), x.1.specialize(&mappings), x.2.clone()))
                                                  .collect(),
                            tuple_struct: aliased.tuple_struct,
                            generic_params: self.generic_params.clone(),
                            documentation: aliased.documentation.clone(),
                        })))
                    }
                    Item::Enum(ref aliased) => {
                        Ok(Some(Item::Enum(Enum {
                            name: self.name.clone(),
                            repr: aliased.repr.clone(),
                            annotations: self.annotations.clone(),
                            values: aliased.values.clone(),
                            documentation: aliased.documentation.clone(),
                        })))
                    }
                    Item::Typedef(ref aliased) => {
                        Ok(Some(Item::Typedef(Typedef {
                            name: self.name.clone(),
                            annotations: self.annotations.clone(),
                            aliased: aliased.aliased.clone(),
                            documentation: self.documentation.clone(),
                        })))
                    }
                    Item::Specialization(ref aliased) => {
                        if self.aliased.generics.len() !=
                           aliased.generic_params.len() {
                            return Err(format!("incomplete specialization"));
                        }

                        let mappings = aliased.generic_params.iter()
                                                             .zip(self.aliased.generics.iter())
                                                             .collect::<Vec<_>>();

                        let generics = aliased.aliased.generics.iter()
                                                               .map(|x| x.specialize(&mappings))
                                                               .collect();

                        Specialization {
                            name: self.name.clone(),
                            annotations: self.annotations.clone(),
                            aliased: GenericPath::new(aliased.aliased.name.clone(), generics),
                            generic_params: self.generic_params.clone(),
                            documentation: self.documentation.clone(),
                        }.specialize(library)
                    }
                }
            }
            None => {
                Err(format!("couldn't find aliased type"))
            }
        }
    }
}

/// A type alias that is represented as a C typedef
#[derive(Debug, Clone)]
pub struct Typedef {
    pub name: String,
    pub annotations: AnnotationSet,
    pub aliased: Type,
    pub documentation: Documentation,
}

impl Typedef {
    pub fn load(name: String,
                annotations: AnnotationSet,
                ty: &syn::Ty,
                doc: String) -> Result<Typedef, String> {
        if let Some(x) = Type::load(ty)? {
            Ok(Typedef {
                name: name,
                annotations: annotations,
                aliased: x,
                documentation: Documentation::load(doc),
            })
        } else {
            Err(format!("cannot have a typedef of a zero sized type"))
        }
    }

    pub fn transfer_annotations(&mut self, out: &mut HashMap<Path, AnnotationSet>) {
        if self.annotations.is_empty() {
            return;
        }

        match self.aliased.get_root_path() {
            Some(alias_path) => {
                if out.contains_key(&alias_path) {
                    warn!("multiple typedef's with annotations for {}. ignoring annotations from {}.",
                          alias_path, self.name);
                    return;
                }

                out.insert(alias_path, self.annotations.clone());
                self.annotations = AnnotationSet::new();
            }
            None => { }
        }
    }

    pub fn add_dependencies(&self, library: &Library, out: &mut DependencyList) {
        self.aliased.add_dependencies(library, out);
    }

    pub fn add_monomorphs(&self, library: &Library, out: &mut Monomorphs) {
        self.aliased.add_monomorphs(library, out);
    }

    pub fn mangle_paths(&mut self, monomorphs: &Monomorphs) {
        self.aliased.mangle_paths(monomorphs);
    }
}

impl Source for Typedef {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        if config.documentation {
            self.documentation.write(config, out);
        }
        out.write("typedef ");
        (self.name.clone(), self.aliased.clone()).write(config, out);
        out.write(";");
    }
}
