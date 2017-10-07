/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::BTreeMap;
use std::mem;

use bindgen::config::Config;
use bindgen::dependencies::Dependencies;
use bindgen::ir::{AnnotationSet, Cfg, Enum, OpaqueItem, Specialization, Struct, Type, Typedef};
use bindgen::library::Library;
use bindgen::monomorph::Monomorphs;

/// An item is any type of rust item besides a function
pub trait Item {
    fn name(&self) -> &str;
    fn cfg(&self) -> &Option<Cfg>;
    fn annotations(&self) -> &AnnotationSet;
    fn annotations_mut(&mut self) -> &mut AnnotationSet;

    fn container(&self) -> ItemContainer;

    fn specialize(&self, library: &Library, aliasee: &Specialization) -> Result<Box<Item>, String>;

    fn rename_for_config(&mut self, _config: &Config) { }
    fn add_dependencies(&self, _library: &Library, _out: &mut Dependencies) { }
    fn instantiate_monomorph(&self, _generics: &Vec<Type>, _library: &Library, _out: &mut Monomorphs) { }
}

#[derive(Debug, Clone)]
pub enum ItemContainer {
    OpaqueItem(OpaqueItem),
    Struct(Struct),
    Enum(Enum),
    Typedef(Typedef),
    Specialization(Specialization),
}

impl ItemContainer {
    pub fn deref(&self) -> &Item {
        match self {
            &ItemContainer::OpaqueItem(ref x) => x,
            &ItemContainer::Struct(ref x) => x,
            &ItemContainer::Enum(ref x) => x,
            &ItemContainer::Typedef(ref x) => x,
            &ItemContainer::Specialization(ref x) => x,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ItemValue<T: Item> {
    Cfg(Vec<T>),
    Single(T)
}

#[derive(Debug, Clone)]
pub struct ItemMap<T: Item> {
    data: BTreeMap<String, ItemValue<T>>,
}

impl<T: Item> ItemMap<T> {
    pub fn new() -> ItemMap<T> {
        ItemMap {
            data: BTreeMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn try_insert(&mut self, item: T) -> bool {
        match (item.cfg().is_some(), self.data.get_mut(item.name())) {
            (true, Some(&mut ItemValue::Cfg(ref mut items))) => {
                items.push(item);
                return true;
            }
            (false, Some(&mut ItemValue::Cfg(_))) => {
                return false;
            }
            (true, Some(&mut ItemValue::Single(_))) => {
                return false;
            }
            (false, Some(&mut ItemValue::Single(_))) => {
                return false;
            }
            _ => { }
        }

        if item.cfg().is_some() {
            self.data.insert(item.name().to_owned(),
                             ItemValue::Cfg(vec![item]));
        } else {
            self.data.insert(item.name().to_owned(),
                             ItemValue::Single(item));
        }

        true
    }

    pub fn get_items(&self, name: &str) -> Option<Vec<ItemContainer>> {
        match self.data.get(name) {
            Some(&ItemValue::Cfg(ref items)) => {
                Some(items.iter()
                          .map(|x| x.container())
                          .collect())
            }
            Some(&ItemValue::Single(ref item)) => {
                Some(vec![item.container()])
            }
            None => None,
        }
    }

    pub fn filter<F>(&mut self, callback: F)
        where F: Fn(&T) -> bool
    {
        let data = mem::replace(&mut self.data, BTreeMap::new());

        for (name, container) in data {
            match container {
                ItemValue::Cfg(items) => {
                    let mut new_items = Vec::new();
                    for item in items {
                        if !callback(&item) {
                            new_items.push(item);
                        }
                    }
                    if new_items.len() > 0 {
                        self.data.insert(name, ItemValue::Cfg(new_items));
                    }
                }
                ItemValue::Single(item) => {
                    if !callback(&item) {
                        self.data.insert(name, ItemValue::Single(item));
                    }
                }
            }
        }
    }

    pub fn for_all_items<F>(&self, mut callback: F)
        where F: FnMut(&T)
    {
        for container in self.data.values() {
            match container {
                &ItemValue::Cfg(ref items) => {
                    for item in items {
                        callback(item);
                    }
                }
                &ItemValue::Single(ref item) => {
                    callback(item)
                }
            }
        }
    }

    pub fn for_all_items_mut<F>(&mut self, mut callback: F)
        where F: FnMut(&mut T)
    {
        for container in self.data.values_mut() {
            match container {
                &mut ItemValue::Cfg(ref mut items) => {
                    for item in items {
                        callback(item);
                    }
                }
                &mut ItemValue::Single(ref mut item) => {
                    callback(item)
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn for_items<F>(&self, name: &str, mut callback: F)
        where F: FnMut(&T)
    {
        match self.data.get(name) {
            Some(&ItemValue::Cfg(ref items)) => {
                for item in items {
                    callback(item);
                }
            }
            Some(&ItemValue::Single(ref item)) => {
                callback(item);
            }
            None => { }
        }
    }

    pub fn for_items_mut<F>(&mut self, name: &str, mut callback: F)
        where F: FnMut(&mut T)
    {
        match self.data.get_mut(name) {
            Some(&mut ItemValue::Cfg(ref mut items)) => {
                for item in items {
                    callback(item);
                }
            }
            Some(&mut ItemValue::Single(ref mut item)) => {
                callback(item);
            }
            None => { }
        }
    }
}
