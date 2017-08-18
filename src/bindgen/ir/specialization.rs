/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use syn;

use bindgen::ir::{AnnotationSet, Cfg, Documentation, Enum};
use bindgen::ir::{GenericPath, Item, OpaqueItem, PrimitiveType, Struct, Typedef};
use bindgen::library::Library;

/// A type alias that generates a copy of its aliasee with a new name. If the type
/// alias has generic values, it specializes its aliasee. This is useful for
/// presenting an interface that includes generic types without mangling.
#[derive(Debug, Clone)]
pub struct Specialization {
    pub name: String,
    pub generic_params: Vec<String>,
    pub aliased: GenericPath,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}

impl Specialization {
    pub fn load(name: String,
                generics: &syn::Generics,
                ty: &syn::Ty,
                attrs: &Vec<syn::Attribute>,
                mod_cfg: &Option<Cfg>) -> Result<Specialization, String>
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
                    generic_params: generic_params,
                    aliased: path,
                    cfg: Cfg::append(mod_cfg, Cfg::load(attrs)),
                    annotations: AnnotationSet::load(attrs)?,
                    documentation: Documentation::load(attrs),
                })
            }
            _ => {
                Err(format!("not a path"))
            }
        }
    }

    pub fn specialize(&self, library: &Library) -> Result<Option<Item>, String> {
        match library.get_item(&self.aliased.name) {
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
                            cfg: self.cfg.clone(),
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
                            generic_params: self.generic_params.clone(),
                            fields: aliased.fields.iter()
                                                  .map(|x| (x.0.clone(), x.1.specialize(&mappings), x.2.clone()))
                                                  .collect(),
                            tuple_struct: aliased.tuple_struct,
                            cfg: self.cfg.clone(),
                            annotations: self.annotations.clone(),
                            documentation: self.documentation.clone(),
                        })))
                    }
                    Item::Enum(ref aliased) => {
                        Ok(Some(Item::Enum(Enum {
                            name: self.name.clone(),
                            repr: aliased.repr.clone(),
                            values: aliased.values.clone(),
                            cfg: self.cfg.clone(),
                            annotations: self.annotations.clone(),
                            documentation: self.documentation.clone(),
                        })))
                    }
                    Item::Typedef(ref aliased) => {
                        Ok(Some(Item::Typedef(Typedef {
                            name: self.name.clone(),
                            aliased: aliased.aliased.clone(),
                            cfg: self.cfg.clone(),
                            annotations: self.annotations.clone(),
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
                            generic_params: self.generic_params.clone(),
                            aliased: GenericPath::new(aliased.aliased.name.clone(), generics),
                            cfg: self.cfg.clone(),
                            annotations: self.annotations.clone(),
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
