/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;
use std::path::PathBuf;

use crate::bindgen::bindings::Bindings;
use crate::bindgen::config::{Config, Language, SortKey};
use crate::bindgen::declarationtyperesolver::DeclarationTypeResolver;
use crate::bindgen::dependencies::Dependencies;
use crate::bindgen::error::Error;
use crate::bindgen::ir::{
    Constant, Enum, Function, Item, ItemContainer, ItemMap, TransparentTypeEraser,
};
use crate::bindgen::ir::{OpaqueItem, Path, Static, Struct, Typedef, Union};
use crate::bindgen::monomorph::Monomorphs;
use crate::bindgen::ItemType;

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
    source_files: Vec<PathBuf>,
    package_version: String,
}

impl Library {
    #[allow(clippy::too_many_arguments)]
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
        source_files: Vec<PathBuf>,
        package_version: String,
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
            source_files,
            package_version,
        }
    }

    pub fn generate(mut self) -> Result<Bindings, Error> {
        let mut eraser = TransparentTypeEraser::default();
        self.erase_transparent_types(&mut eraser);
        self.transfer_annotations();

        match self.config.function.sort_by.unwrap_or(self.config.sort_by) {
            SortKey::Name => self.functions.sort_by(|x, y| x.path.cmp(&y.path)),
            SortKey::None => { /* keep input order */ }
        }

        if self.config.language != Language::Cxx {
            self.instantiate_monomorphs(&mut eraser);
        }
        self.remove_excluded();
        if self.config.language == Language::C {
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
                if dependencies.items.insert(path) {
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
            let mut constants = self.constants.to_vec();
            match self.config.constant.sort_by.unwrap_or(self.config.sort_by) {
                SortKey::Name => constants.sort_by(|x, y| x.path.cmp(&y.path)),
                SortKey::None => { /* keep input order */ }
            }
            constants
        } else {
            vec![]
        };

        let globals = if self.config.export.should_generate(ItemType::Globals) {
            let mut globals = self.globals.to_vec();
            match self.config.constant.sort_by.unwrap_or(self.config.sort_by) {
                SortKey::Name => globals.sort_by(|x, y| x.path.cmp(&y.path)),
                SortKey::None => { /* keep input order */ }
            }
            globals
        } else {
            vec![]
        };
        let functions = if self.config.export.should_generate(ItemType::Functions) {
            self.functions
        } else {
            vec![]
        };

        Ok(Bindings::new(
            self.config,
            self.structs,
            self.typedefs,
            constants,
            globals,
            items,
            functions,
            self.source_files,
            false,
            self.package_version,
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

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    fn erase_transparent_types_for_items<T: Item + Clone>(
        &self,
        eraser: &mut TransparentTypeEraser,
        items: &ItemMap<T>,
    ) -> ItemMap<T> {
        // NOTE: Because `items` is actually a shared reference to `self`, we cannot take it as
        // mutable. We also cannot `drain` or `take` it first, because then the items would be
        // unavailable for lookup during the type erasure process. So we mutate a clone, and let the
        // caller assign the result back after these shared references have died.
        let mut items = items.clone();
        items.for_all_items_mut(|item| {
            item.erase_transparent_types_inplace(self, eraser, &[]);
        });
        items
    }

    fn erase_transparent_types(&mut self, eraser: &mut TransparentTypeEraser) {
        self.constants = self.erase_transparent_types_for_items(eraser, &self.constants);
        self.globals = self.erase_transparent_types_for_items(eraser, &self.globals);
        self.enums = self.erase_transparent_types_for_items(eraser, &self.enums);
        self.structs = self.erase_transparent_types_for_items(eraser, &self.structs);
        self.unions = self.erase_transparent_types_for_items(eraser, &self.unions);
        self.opaque_items = self.erase_transparent_types_for_items(eraser, &self.opaque_items);
        self.typedefs = self.erase_transparent_types_for_items(eraser, &self.typedefs);

        // Functions do not `impl Item` and are not stored in an `ItemMap`, so do them manually.
        let mut functions = self.functions.clone();
        for f in &mut functions {
            f.erase_transparent_types_inplace(self, eraser);
        }
        self.functions = functions;
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
        if !self.config.style.generate_tag() {
            return;
        }

        let mut resolver = DeclarationTypeResolver::default();

        self.structs.for_all_items(|x| {
            x.collect_declaration_types(&mut resolver);
        });

        self.enums.for_all_items(|x| {
            x.collect_declaration_types(&mut resolver);
        });

        self.unions.for_all_items(|x| {
            x.collect_declaration_types(&mut resolver);
        });

        self.typedefs.for_all_items(|x| {
            x.collect_declaration_types(&mut resolver);
        });

        // NOTE: Intentionally last, so that in case there's an opaque type
        // which is conflicting with a non-opaque one, the later wins.
        self.opaque_items.for_all_items(|x| {
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

    fn instantiate_monomorphs(&mut self, eraser: &mut TransparentTypeEraser) {
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
        for mut monomorph in monomorphs.drain_structs() {
            monomorph.erase_transparent_types_inplace(self, eraser, &[]);
            self.structs.try_insert(monomorph);
        }
        for mut monomorph in monomorphs.drain_unions() {
            monomorph.erase_transparent_types_inplace(self, eraser, &[]);
            self.unions.try_insert(monomorph);
        }
        for mut monomorph in monomorphs.drain_opaques() {
            monomorph.erase_transparent_types_inplace(self, eraser, &[]);
            self.opaque_items.try_insert(monomorph);
        }
        for mut monomorph in monomorphs.drain_typedefs() {
            monomorph.erase_transparent_types_inplace(self, eraser, &[]);
            self.typedefs.try_insert(monomorph);
        }
        for mut monomorph in monomorphs.drain_enums() {
            monomorph.erase_transparent_types_inplace(self, eraser, &[]);
            self.enums.try_insert(monomorph);
        }

        // Remove structs and opaque items that are generic
        self.opaque_items.filter(|x| x.is_generic());
        self.structs.filter(|x| x.is_generic());
        self.unions.filter(|x| x.is_generic());
        self.enums.filter(|x| x.is_generic());
        self.typedefs.filter(|x| x.is_generic());

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
