/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::BTreeMap;
use std::collections::HashMap;

use bindgen::bindings::Bindings;
use bindgen::config::Config;
use bindgen::ir::{Enum, Function, Item, OpaqueItem};
use bindgen::ir::{Path, Struct, Typedef};
use bindgen::dependencies::DependencyList;

#[derive(Debug, Clone)]
pub struct Library {
    config: Config,
    enums: BTreeMap<String, Enum>,
    structs: BTreeMap<String, Struct>,
    opaque_items: BTreeMap<String, OpaqueItem>,
    typedefs: BTreeMap<String, Typedef>,
    functions: Vec<Function>,
}

impl Library {
    pub fn new(config: Config,
               enums: BTreeMap<String, Enum>,
               structs: BTreeMap<String, Struct>,
               opaque_items: BTreeMap<String, OpaqueItem>,
               typedefs: BTreeMap<String, Typedef>,
               functions: Vec<Function>) -> Library {
        Library {
            config: config,
            enums: enums,
            structs: structs,
            opaque_items: opaque_items,
            typedefs: typedefs,
            functions: functions,
        }
    }

    pub fn generate(mut self) -> Result<Bindings, String> {
        self.transfer_annotations();

        let deps = DependencyList::new(&self.functions, &self, &self.config);
        deps.print();

        // Gather only the items that we need for this
        // `extern "c"` interface
        Ok(Bindings::new(self.config, deps.calculate_order()))
    }

    pub fn get_item(&self, p: &Path) -> Option<Item> {
        if let Some(x) = self.enums.get(p) {
            return Some(Item::Enum(x.clone()));
        }
        if let Some(x) = self.structs.get(p) {
            return Some(Item::Struct(x.clone()));
        }
        if let Some(x) = self.opaque_items.get(p) {
            return Some(Item::OpaqueItem(x.clone()));
        }
        if let Some(x) = self.typedefs.get(p) {
            return Some(Item::Typedef(x.clone()));
        }

        None
    }

    fn transfer_annotations(&mut self) {
        let mut annotations = HashMap::new();

        for (_, ref mut typedef) in &mut self.typedefs {
            typedef.transfer_annotations(&mut annotations);
        }

        for (alias_path, annotations) in annotations {
            // TODO
            if let Some(x) = self.enums.get_mut(&alias_path) {
                if !x.annotations.is_empty() {
                    warn!("can't transfer annotations from typedef to alias ({}) that already has annotations.",
                          alias_path);
                    continue;
                }
                x.annotations = annotations;
                continue;
            }
            if let Some(x) = self.structs.get_mut(&alias_path) {
                if !x.annotations.is_empty() {
                    warn!("can't transfer annotations from typedef to alias ({}) that already has annotations.",
                          alias_path);
                    continue;
                }
                x.annotations = annotations;
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
        }
    }
}
