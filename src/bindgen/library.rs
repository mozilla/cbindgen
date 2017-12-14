/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;
use std::mem;

use bindgen::bindings::Bindings;
use bindgen::config::{Config, Language};
use bindgen::dependencies::Dependencies;
use bindgen::ir::{Constant, Enum, Function, Item, ItemContainer, ItemMap};
use bindgen::ir::{OpaqueItem, Path, Static, Struct, Typedef, Union};
use bindgen::monomorph::Monomorphs;

#[derive(Debug, Clone)]
pub struct Library {
    config: Config,
    constants: ItemMap<Constant>,
    globals: ItemMap<Static>,
    enums: ItemMap<Enum>,
    structs: ItemMap<Struct>,
    unions: ItemMap<Union>,
    opaque_items: ItemMap<OpaqueItem>,
    typedefs: ItemMap<Typedef>,
    functions: Vec<Function>,
}

impl Library {
    pub fn new(
        config: Config,
        constants: ItemMap<Constant>,
        globals: ItemMap<Static>,
        enums: ItemMap<Enum>,
        structs: ItemMap<Struct>,
        unions: ItemMap<Union>,
        opaque_items: ItemMap<OpaqueItem>,
        typedefs: ItemMap<Typedef>,
        functions: Vec<Function>,
    ) -> Library {
        Library {
            config: config,
            constants: constants,
            globals: globals,
            enums: enums,
            structs: structs,
            unions: unions,
            opaque_items: opaque_items,
            typedefs: typedefs,
            functions: functions,
        }
    }

    pub fn generate(mut self) -> Result<Bindings, String> {
        self.functions.sort_by(|x, y| x.name.cmp(&y.name));

        self.transfer_annotations();
        self.rename_items();
        self.simplify_option_to_ptr();

        if self.config.language == Language::C {
            self.instantiate_monomorphs();
        }

        let mut dependencies = Dependencies::new();

        for function in &self.functions {
            function.add_dependencies(&self, &mut dependencies);
        }
        self.globals.for_all_items(|global| {
            global.add_dependencies(&self, &mut dependencies);
        });

        dependencies.sort();

        let items = dependencies.order;
        let constants = self.constants.to_vec();
        let globals = self.globals.to_vec();
        let functions = mem::replace(&mut self.functions, Vec::new());

        Ok(Bindings::new(
            self.config.clone(),
            constants,
            globals,
            items,
            functions,
        ))
    }

    pub fn get_items(&self, p: &Path) -> Option<Vec<ItemContainer>> {
        if let Some(x) = self.enums.get_items(p) {
            return Some(x);
        }
        if let Some(x) = self.structs.get_items(p) {
            return Some(x);
        }
        if let Some(x) = self.unions.get_items(p) {
            return Some(x);
        }
        if let Some(x) = self.opaque_items.get_items(p) {
            return Some(x);
        }
        if let Some(x) = self.typedefs.get_items(p) {
            return Some(x);
        }

        None
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
                    warn!(
                        "Can't transfer annotations from typedef to alias ({}) \
                         that already has annotations.",
                        alias_path
                    );
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
                    warn!(
                        "Can't transfer annotations from typedef to alias ({}) \
                         that already has annotations.",
                        alias_path
                    );
                }
            });
            if transferred {
                continue;
            }
            self.unions.for_items_mut(&alias_path, |x| {
                if x.annotations().is_empty() {
                    *x.annotations_mut() = annotations.clone();
                    transferred = true;
                } else {
                    warn!(
                        "Can't transfer annotations from typedef to alias ({}) \
                         that already has annotations.",
                        alias_path
                    );
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
                    warn!(
                        "Can't transfer annotations from typedef to alias ({}) \
                         that already has annotations.",
                        alias_path
                    );
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
                    warn!(
                        "Can't transfer annotations from typedef to alias ({}) \
                         that already has annotations.",
                        alias_path
                    );
                }
            });
            if transferred {
                continue;
            }
        }
    }

    fn rename_items(&mut self) {
        let config = &self.config;
        self.structs
            .for_all_items_mut(|x| x.rename_for_config(config));
        self.unions
            .for_all_items_mut(|x| x.rename_for_config(config));
        self.enums
            .for_all_items_mut(|x| x.rename_for_config(config));

        for item in &mut self.functions {
            item.rename_for_config(&self.config);
        }
    }

    fn simplify_option_to_ptr(&mut self) {
        self.structs.for_all_items_mut(|x| {
            x.simplify_option_to_ptr();
        });
        self.unions.for_all_items_mut(|x| {
            x.simplify_option_to_ptr();
        });
        self.globals.for_all_items_mut(|x| {
            x.simplify_option_to_ptr();
        });
        self.typedefs.for_all_items_mut(|x| {
            x.simplify_option_to_ptr();
        });
        for x in &mut self.functions {
            x.simplify_option_to_ptr();
        }
    }

    fn instantiate_monomorphs(&mut self) {
        // Collect a list of monomorphs
        let mut monomorphs = Monomorphs::default();

        self.structs.for_all_items(|x| {
            x.add_monomorphs(self, &mut monomorphs);
        });
        self.unions.for_all_items(|x| {
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
        for monomorph in monomorphs.drain_unions() {
            self.unions.try_insert(monomorph);
        }
        for monomorph in monomorphs.drain_opaques() {
            self.opaque_items.try_insert(monomorph);
        }
        for monomorph in monomorphs.drain_typedefs() {
            self.typedefs.try_insert(monomorph);
        }

        // Remove structs and opaque items that are generic
        self.opaque_items.filter(|x| x.generic_params.len() > 0);
        self.structs.filter(|x| x.generic_params.len() > 0);
        self.unions.filter(|x| x.generic_params.len() > 0);
        self.typedefs.filter(|x| x.generic_params.len() > 0);

        // Mangle the paths that remain
        self.unions
            .for_all_items_mut(|x| x.mangle_paths(&monomorphs));
        self.structs
            .for_all_items_mut(|x| x.mangle_paths(&monomorphs));
        self.typedefs
            .for_all_items_mut(|x| x.mangle_paths(&monomorphs));
        for x in &mut self.functions {
            x.mangle_paths(&monomorphs);
        }
    }
}
