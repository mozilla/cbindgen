/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;
use std::mem;

use bindgen::bindings::Bindings;
use bindgen::config::{Config, Language};
use bindgen::dependencies::Dependencies;
use bindgen::ir::{Enum, Function, ItemContainer, ItemMap, Item, OpaqueItem};
use bindgen::ir::{Path, Specialization, Struct, Typedef};
use bindgen::monomorph::{Monomorphs, TemplateSpecialization};

#[derive(Debug, Clone)]
pub struct Library {
    config: Config,
    enums: ItemMap<Enum>,
    structs: ItemMap<Struct>,
    opaque_items: ItemMap<OpaqueItem>,
    typedefs: ItemMap<Typedef>,
    specializations: ItemMap<Specialization>,
    functions: Vec<Function>,
    template_specializations: Vec<TemplateSpecialization>,
}

impl Library {
    pub fn new(config: Config,
               enums: ItemMap<Enum>,
               structs: ItemMap<Struct>,
               opaque_items: ItemMap<OpaqueItem>,
               typedefs: ItemMap<Typedef>,
               specializations: ItemMap<Specialization>,
               functions: Vec<Function>) -> Library {
        Library {
            config: config,
            enums: enums,
            structs: structs,
            opaque_items: opaque_items,
            typedefs: typedefs,
            specializations: specializations,
            functions: functions,
            template_specializations: Vec::new(),
        }
    }

    pub fn generate(mut self) -> Result<Bindings, String> {
        self.transfer_annotations();
        self.rename_items();
        self.specialize_items();
        self.instantiate_monomorphs();

        let mut dependencies = Dependencies::new();

        for function in &self.functions {
            function.add_dependencies(&self, &mut dependencies);
        }

        if self.config.structure.generic_template_specialization &&
           self.config.language == Language::Cxx {
            for template_specialization in &self.template_specializations {
              template_specialization.add_dependencies(&self, &mut dependencies);
            }
        }

        dependencies.sort();

        let items = dependencies.order;
        let functions = mem::replace(&mut self.functions, Vec::new());
        let template_specializations = mem::replace(&mut self.template_specializations, Vec::new());

        Ok(Bindings::new(self.config.clone(),
                         items,
                         functions,
                         template_specializations))
    }

    pub fn get_items(&self, p: &Path) -> Option<Vec<ItemContainer>> {
        if let Some(x) = self.enums.get_items(p) {
            return Some(x);
        }
        if let Some(x) = self.structs.get_items(p) {
            return Some(x);
        }
        if let Some(x) = self.opaque_items.get_items(p) {
            return Some(x);
        }
        if let Some(x) = self.typedefs.get_items(p) {
            return Some(x);
        }
        if let Some(x) = self.specializations.get_items(p) {
            return Some(x);
        }

        None
    }

    fn insert_item(&mut self, item: ItemContainer) {
        match item {
            ItemContainer::OpaqueItem(x) => {
                self.opaque_items.try_insert(x);
            },
            ItemContainer::Struct(x) => {
                self.structs.try_insert(x);
            },
            ItemContainer::Enum(x) => {
                self.enums.try_insert(x);
            },
            ItemContainer::Typedef(x) => {
                self.typedefs.try_insert(x);
            },
            ItemContainer::Specialization(x) => {
                self.specializations.try_insert(x);
            },
        };
    }

    fn transfer_annotations(&mut self) {
        let mut annotations = HashMap::new();

        self.typedefs.for_all_items_mut(|x| {
            x.transfer_annotations(&mut annotations);
        });

        for (alias_path, annotations) in annotations {
            // TODO
            let mut transferred = false;

            self.enums.for_items_mut(&alias_path, |x| {
                if x.annotations().is_empty() {
                    *x.annotations_mut() = annotations.clone();
                    transferred = true;
                } else {
                    warn!("Can't transfer annotations from typedef to alias ({}) that already has annotations.",
                          alias_path);
                }
            });
            if transferred {
                continue;
            }
            self.structs.for_items_mut(&alias_path, |x| {
                if x.annotations().is_empty() {
                    *x.annotations_mut() = annotations.clone();
                    transferred = true;
                } else {
                    warn!("Can't transfer annotations from typedef to alias ({}) that already has annotations.",
                          alias_path);
                }
            });
            if transferred {
                continue;
            }
            self.opaque_items.for_items_mut(&alias_path, |x| {
                if x.annotations().is_empty() {
                    *x.annotations_mut() = annotations.clone();
                    transferred = true;
                } else {
                    warn!("Can't transfer annotations from typedef to alias ({}) that already has annotations.",
                          alias_path);
                }
            });
            if transferred {
                continue;
            }
            self.specializations.for_items_mut(&alias_path, |x| {
                if x.annotations().is_empty() {
                    *x.annotations_mut() = annotations.clone();
                    transferred = true;
                } else {
                    warn!("Can't transfer annotations from typedef to alias ({}) that already has annotations.",
                          alias_path);
                }
            });
            if transferred {
                continue;
            }
            self.typedefs.for_items_mut(&alias_path, |x| {
                if x.annotations().is_empty() {
                    *x.annotations_mut() = annotations.clone();
                    transferred = true;
                } else {
                    warn!("Can't transfer annotations from typedef to alias ({}) that already has annotations.",
                          alias_path);
                }
            });
            if transferred {
                continue;
            }
        }
    }

    fn rename_items(&mut self) {
        let config = &self.config;
        self.structs.for_all_items_mut(|x| x.rename_for_config(config));
        self.enums.for_all_items_mut(|x| x.rename_for_config(config));

        for item in &mut self.functions {
            item.rename_for_config(&self.config);
        }
    }

    fn specialize_items(&mut self) {
        let mut specializations = Vec::new();

        self.specializations.for_all_items(|x| {
            match x.resolve_specialization(&self) {
                Ok(specialization) => {
                    specializations.push(specialization);
                }
                Err(msg) => {
                    warn!("Specializing {} failed - ({}).", x.name.clone(), msg);
                }
            }
        });

        for specialization in specializations {
            self.insert_item(specialization.container());
        }

        self.specializations.clear();
    }

    fn instantiate_monomorphs(&mut self) {
        assert!(self.specializations.len() == 0);

        // Collect a list of monomorphs
        let mut monomorphs = Monomorphs::new();

        self.structs.for_all_items(|x| {
            x.add_monomorphs(self, &mut monomorphs);
        });
        self.typedefs.for_all_items(|x| {
            x.add_monomorphs(self, &mut monomorphs);
        });
        for x in &self.functions {
            x.add_monomorphs(self, &mut monomorphs);
        }

        // Insert the monomorphs into self
        for monomorph in monomorphs.drain_structs() {
            self.structs.try_insert(monomorph);
        }
        for monomorph in monomorphs.drain_opaques() {
            self.opaque_items.try_insert(monomorph);
        }

        // Remove structs and opaque items that are generic
        self.opaque_items.filter(|x| x.generic_params.len() > 0);
        self.structs.filter(|x| x.generic_params.len() > 0);

        // Mangle the paths that remain
        self.structs.for_all_items_mut(|x| x.mangle_paths(&monomorphs));
        self.typedefs.for_all_items_mut(|x| x.mangle_paths(&monomorphs));
        for x in &mut self.functions {
            x.mangle_paths(&monomorphs);
        }

        self.template_specializations = monomorphs.drain_template_specializations();
    }
}
