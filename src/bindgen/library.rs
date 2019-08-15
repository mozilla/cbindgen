/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;
use std::mem;

use bindgen::bindings::Bindings;
use bindgen::config::{Config, Language};
use bindgen::declarationtyperesolver::DeclarationTypeResolver;
use bindgen::dependencies::Dependencies;
use bindgen::error::Error;
use bindgen::ir::{Constant, Enum, Function, Item, ItemContainer, ItemMap};
use bindgen::ir::{OpaqueItem, Path, Static, Struct, Typedef, Union};
use bindgen::monomorph::Monomorphs;
use bindgen::ItemType;

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
            config,
            constants,
            globals,
            enums,
            structs,
            unions,
            opaque_items,
            typedefs,
            functions,
        }
    }

    pub fn generate(mut self) -> Result<Bindings, Error> {
        self.remove_excluded();
        self.functions.sort_by(|x, y| x.path.cmp(&y.path));
        self.transfer_annotations();
        self.simplify_standard_types();

        if self.config.language == Language::C {
            self.instantiate_monomorphs();
            self.resolve_declaration_types();
        }

        self.rename_items();

        let mut dependencies = Dependencies::new();

        for function in &self.functions {
            function.add_dependencies(&self, &mut dependencies);
        }
        self.globals.for_all_items(|global| {
            global.add_dependencies(&self, &mut dependencies);
        });
        self.constants.for_all_items(|constant| {
            constant.add_dependencies(&self, &mut dependencies);
        });
        for name in &self.config.export.include {
            let path = Path::new(name.clone());
            if let Some(items) = self.get_items(&path) {
                if !dependencies.items.contains(&path) {
                    dependencies.items.insert(path);

                    for item in &items {
                        item.deref().add_dependencies(&self, &mut dependencies);
                    }
                    for item in items {
                        dependencies.order.push(item);
                    }
                }
            }
        }

        dependencies.sort();

        let items = dependencies.order;
        let constants = if self.config.export.should_generate(ItemType::Constants) {
            self.constants.to_vec()
        } else {
            vec![]
        };

        let globals = if self.config.export.should_generate(ItemType::Globals) {
            self.globals.to_vec()
        } else {
            vec![]
        };
        let functions = if self.config.export.should_generate(ItemType::Functions) {
            mem::replace(&mut self.functions, vec![])
        } else {
            vec![]
        };

        Ok(Bindings::new(
            self.config,
            self.structs,
            constants,
            globals,
            items,
            functions,
        ))
    }

    pub fn get_items(&self, p: &Path) -> Option<Vec<ItemContainer>> {
        macro_rules! find {
            ($field:ident, $kind:ident) => {
                if self.config.export.should_generate(ItemType::$kind) {
                    if let Some(x) = self.$field.get_items(p) {
                        return Some(x);
                    }
                }
            };
        }

        find!(enums, Enums);
        find!(structs, Structs);
        find!(unions, Unions);
        find!(opaque_items, OpaqueItems);
        find!(typedefs, Typedefs);

        None
    }

    fn remove_excluded(&mut self) {
        let config = &self.config;
        // FIXME: interpret `config.export.exclude` as `Path`s.
        self.functions
            .retain(|x| !config.export.exclude.iter().any(|y| y == x.path().name()));
        self.enums
            .filter(|x| config.export.exclude.iter().any(|y| y == x.path().name()));
        self.structs
            .filter(|x| config.export.exclude.iter().any(|y| y == x.path().name()));
        self.unions
            .filter(|x| config.export.exclude.iter().any(|y| y == x.path().name()));
        self.opaque_items
            .filter(|x| config.export.exclude.iter().any(|y| y == x.path().name()));
        self.typedefs
            .filter(|x| config.export.exclude.iter().any(|y| y == x.path().name()));
        self.globals
            .filter(|x| config.export.exclude.iter().any(|y| y == x.path().name()));
        self.constants
            .filter(|x| config.export.exclude.iter().any(|y| y == x.path().name()));
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

        self.globals
            .for_all_items_mut(|x| x.rename_for_config(config));
        self.globals.rebuild();

        self.constants
            .for_all_items_mut(|x| x.rename_for_config(config));
        self.constants.rebuild();

        self.structs
            .for_all_items_mut(|x| x.rename_for_config(config));
        self.structs.rebuild();

        self.unions
            .for_all_items_mut(|x| x.rename_for_config(config));
        self.unions.rebuild();

        self.enums
            .for_all_items_mut(|x| x.rename_for_config(config));
        self.enums.rebuild();

        self.opaque_items
            .for_all_items_mut(|x| x.rename_for_config(config));
        self.opaque_items.rebuild();

        self.typedefs
            .for_all_items_mut(|x| x.rename_for_config(config));
        self.typedefs.rebuild();

        for item in &mut self.functions {
            item.rename_for_config(&self.config);
        }
    }

    fn resolve_declaration_types(&mut self) {
        if self.config.style.generate_typedef() {
            return;
        }

        let mut resolver = DeclarationTypeResolver::new();

        self.structs.for_all_items(|x| {
            x.collect_declaration_types(&mut resolver);
        });

        self.opaque_items.for_all_items(|x| {
            x.collect_declaration_types(&mut resolver);
        });

        self.enums.for_all_items(|x| {
            x.collect_declaration_types(&mut resolver);
        });

        self.unions.for_all_items(|x| {
            x.collect_declaration_types(&mut resolver);
        });

        self.enums
            .for_all_items_mut(|x| x.resolve_declaration_types(&resolver));

        self.structs
            .for_all_items_mut(|x| x.resolve_declaration_types(&resolver));

        self.unions
            .for_all_items_mut(|x| x.resolve_declaration_types(&resolver));

        self.typedefs
            .for_all_items_mut(|x| x.resolve_declaration_types(&resolver));

        self.globals
            .for_all_items_mut(|x| x.resolve_declaration_types(&resolver));

        for item in &mut self.functions {
            item.resolve_declaration_types(&resolver);
        }
    }

    fn simplify_standard_types(&mut self) {
        self.structs.for_all_items_mut(|x| {
            x.simplify_standard_types();
        });
        self.unions.for_all_items_mut(|x| {
            x.simplify_standard_types();
        });
        self.globals.for_all_items_mut(|x| {
            x.simplify_standard_types();
        });
        self.typedefs.for_all_items_mut(|x| {
            x.simplify_standard_types();
        });
        for x in &mut self.functions {
            x.simplify_standard_types();
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
        self.enums.for_all_items(|x| {
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
        for monomorph in monomorphs.drain_enums() {
            self.enums.try_insert(monomorph);
        }

        // Remove structs and opaque items that are generic
        self.opaque_items.filter(|x| x.generic_params.len() > 0);
        self.structs.filter(|x| x.generic_params.len() > 0);
        self.unions.filter(|x| x.generic_params.len() > 0);
        self.enums.filter(|x| x.generic_params.len() > 0);
        self.typedefs.filter(|x| x.generic_params.len() > 0);

        // Mangle the paths that remain
        self.unions
            .for_all_items_mut(|x| x.mangle_paths(&monomorphs));
        self.structs
            .for_all_items_mut(|x| x.mangle_paths(&monomorphs));
        self.enums
            .for_all_items_mut(|x| x.mangle_paths(&monomorphs));
        self.typedefs
            .for_all_items_mut(|x| x.mangle_paths(&monomorphs));
        for x in &mut self.functions {
            x.mangle_paths(&monomorphs);
        }
    }
}
