/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::mem;

use bindgen::bindings::Bindings;
use bindgen::config::{Config, Language};
use bindgen::dependencies::Dependencies;
use bindgen::ir::{Enum, Function, Item, OpaqueItem};
use bindgen::ir::{Path, Specialization, Struct, Typedef};
use bindgen::monomorph::{Monomorphs, TemplateSpecialization};

#[derive(Debug, Clone)]
pub struct Library {
    config: Config,
    enums: BTreeMap<String, Vec<Enum>>,
    structs: BTreeMap<String, Vec<Struct>>,
    opaque_items: BTreeMap<String, OpaqueItem>,
    typedefs: BTreeMap<String, Typedef>,
    specializations: BTreeMap<String, Specialization>,
    functions: Vec<Function>,
    template_specializations: Vec<TemplateSpecialization>,
}

impl Library {
    pub fn new(config: Config,
               enums: BTreeMap<String, Vec<Enum>>,
               structs: BTreeMap<String, Vec<Struct>>,
               opaque_items: BTreeMap<String, OpaqueItem>,
               typedefs: BTreeMap<String, Typedef>,
               specializations: BTreeMap<String, Specialization>,
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

    pub fn get_item(&self, p: &Path) -> Option<Vec<Item>> {
        if let Some(x) = self.enums.get(p) {
            return Some(x.iter().map(|x| Item::Enum(x.clone())).collect());
        }
        if let Some(x) = self.structs.get(p) {
            return Some(x.iter().map(|x| Item::Struct(x.clone())).collect());
        }
        if let Some(x) = self.opaque_items.get(p) {
            return Some(vec![Item::OpaqueItem(x.clone())]);
        }
        if let Some(x) = self.typedefs.get(p) {
            return Some(vec![Item::Typedef(x.clone())]);
        }
        if let Some(x) = self.specializations.get(p) {
            return Some(vec![Item::Specialization(x.clone())]);
        }

        None
    }

    fn insert_item(&mut self, item: Item) {
        match item {
            Item::OpaqueItem(x) => { self.opaque_items.insert(x.name.clone(), x); },
            Item::Struct(x) => {
                if !self.structs.contains_key(&x.name) {
                    self.structs.insert(x.name.clone(), Vec::new());
                }
                let structs = self.structs.get_mut(&x.name).unwrap();
                if structs.len() == 0 ||
                   x.cfg.is_some() {
                    structs.push(x);
                }
            },
            Item::Enum(x) => {
                if !self.enums.contains_key(&x.name) {
                    self.enums.insert(x.name.clone(), Vec::new());
                }
                let enums = self.enums.get_mut(&x.name).unwrap();
                if enums.len() == 0 ||
                   x.cfg.is_some() {
                    enums.push(x);
                }
            },
            Item::Typedef(x) => { self.typedefs.insert(x.name.clone(), x); },
            Item::Specialization(x) => { self.specializations.insert(x.name.clone(), x); },
        };
    }

    fn transfer_annotations(&mut self) {
        let mut annotations = HashMap::new();

        for (_, ref mut typedef) in &mut self.typedefs {
            typedef.transfer_annotations(&mut annotations);
        }

        for (alias_path, annotations) in annotations {
            // TODO
            if let Some(x) = self.enums.get_mut(&alias_path) {
                for x in x {
                    if !x.annotations.is_empty() {
                        warn!("can't transfer annotations from typedef to alias ({}) that already has annotations.",
                              alias_path);
                        continue;
                    }
                    x.annotations = annotations.clone();
                }
                continue;
            }
            if let Some(x) = self.structs.get_mut(&alias_path) {
                for x in x {
                    if !x.annotations.is_empty() {
                        warn!("can't transfer annotations from typedef to alias ({}) that already has annotations.",
                              alias_path);
                        continue;
                    }
                    x.annotations = annotations.clone();
                }
                continue;
            }
            if let Some(x) = self.opaque_items.get_mut(&alias_path) {
                if !x.annotations.is_empty() {
                    warn!("can't transfer annotations from typedef to alias ({}) that already has annotations.",
                          alias_path);
                    continue;
                }
                x.annotations = annotations;
                continue;
            }
            if let Some(x) = self.typedefs.get_mut(&alias_path) {
                if !x.annotations.is_empty() {
                    warn!("can't transfer annotations from typedef to alias ({}) that already has annotations.",
                          alias_path);
                    continue;
                }
                x.annotations = annotations;
                continue;
            }
            if let Some(x) = self.specializations.get_mut(&alias_path) {
                if !x.annotations.is_empty() {
                    warn!("can't transfer annotations from typedef to alias ({}) that already has annotations.",
                          alias_path);
                    continue;
                }
                x.annotations = annotations;
                continue;
            }
        }
    }

    fn rename_items(&mut self) {
        for items in self.structs.values_mut() {
            for item in items {
                item.rename_fields(&self.config);
            }
        }

        for items in self.enums.values_mut() {
            for item in items {
                item.rename_values(&self.config);
            }
        }

        for item in &mut self.functions {
            item.rename_args(&self.config);
        }
    }

    fn specialize_items(&mut self) {
        let mut specializations = Vec::new();

        for specialization in self.specializations.values() {
            match specialization.specialize(&self) {
                Ok(Some(specialization)) => {
                    specializations.push(specialization);
                }
                Ok(None) => { }
                Err(msg) => {
                    warn!("specializing {} failed - ({})", specialization.name.clone(), msg);
                }
            }
        }

        for specialization in specializations {
            self.insert_item(specialization);
        }

        self.specializations.clear();
    }

    fn instantiate_monomorphs(&mut self) {
        assert!(self.specializations.len() == 0);

        // Collect a list of monomorphs
        let mut monomorphs = Monomorphs::new();

        for x in self.structs.values() {
            for x in x {
                x.add_monomorphs(self, &mut monomorphs);
            }
        }
        for x in self.typedefs.values() {
            x.add_monomorphs(self, &mut monomorphs);
        }
        for x in &self.functions {
            x.add_monomorphs(self, &mut monomorphs);
        }

        // Insert the monomorphs into self
        for monomorph in monomorphs.drain_structs() {
            if !self.structs.contains_key(&monomorph.name) {
                self.structs.insert(monomorph.name.clone(), Vec::new());
            }
            let structs = self.structs.get_mut(&monomorph.name).unwrap();
            if structs.len() == 0 ||
               monomorph.cfg.is_some() {
                structs.push(monomorph);
            }
        }
        for monomorph in monomorphs.drain_opaques() {
            self.opaque_items.insert(monomorph.name.clone(), monomorph);
        }

        // Remove structs and opaque items that are generic
        let opaque_items = mem::replace(&mut self.opaque_items, BTreeMap::new());
        for (path, item) in opaque_items {
            if item.generic_params.len() != 0 {
                continue;
            }
            self.opaque_items.insert(path, item);
        }

        let structs = mem::replace(&mut self.structs, BTreeMap::new());
        for (path, items) in structs {
            for item in items {
                if item.generic_params.len() != 0 {
                    continue;
                }

                if !self.structs.contains_key(&path) {
                    self.structs.insert(path.clone(), Vec::new());
                }
                let structs = self.structs.get_mut(&path).unwrap();
                if structs.len() == 0 ||
                   item.cfg.is_some() {
                    structs.push(item);
                }
            }
        }

        // Mangle the paths that remain
        for x in self.structs.values_mut() {
            for x in x {
                x.mangle_paths(&monomorphs);
            }
        }
        for x in self.typedefs.values_mut() {
            x.mangle_paths(&monomorphs);
        }
        for x in &mut self.functions {
            x.mangle_paths(&monomorphs);
        }

        self.template_specializations = monomorphs.drain_template_specializations();
    }
}
