/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use syn;

use bindgen::config::Config;
use bindgen::dependencies::Dependencies;
use bindgen::ir::{AnnotationSet, Cfg, Documentation, Item, ItemContainer, Specialization, Type};
use bindgen::library::Library;
use bindgen::writer::{Source, SourceWriter};

#[derive(Debug, Clone)]
pub struct Static {
    pub name: String,
    pub ty: Type,
    pub mutable: bool,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}


impl Static {
    pub fn load(name: String,
                ty: &syn::Ty,
                mutable: &syn::Mutability,
                attrs: &Vec<syn::Attribute>,
                mod_cfg: &Option<Cfg>) -> Result<Static, String>
    {
        let ty = Type::load(ty)?;

        if ty.is_none() {
            return Err("Cannot have a zero sized static definition.".to_owned());
        }

        let ty = ty.unwrap();
        let mutable = mutable == &syn::Mutability::Mutable;

        Ok(Static {
            name: name,
            ty: ty,
            mutable: mutable,
            cfg: Cfg::append(mod_cfg, Cfg::load(attrs)),
            annotations: AnnotationSet::load(attrs)?,
            documentation: Documentation::load(attrs),
        })
    }

    pub fn simplify_option_to_ptr(&mut self) {
        self.ty.simplify_option_to_ptr();
    }
}

impl Item for Static {
    fn name(&self) -> &str {
        &self.name
    }

    fn cfg(&self) -> &Option<Cfg> {
        &self.cfg
    }

    fn annotations(&self) -> &AnnotationSet {
        &self.annotations
    }

    fn annotations_mut(&mut self) -> &mut AnnotationSet {
        &mut self.annotations
    }

    fn container(&self) -> ItemContainer {
        ItemContainer::Static(self.clone())
    }

    fn specialize(&self, _library: &Library, _aliasee: &Specialization) -> Result<Box<Item>, String> {
        unreachable!();
    }

    fn add_dependencies(&self, library: &Library, out: &mut Dependencies) {
        self.ty.add_dependencies(library, out);
    }
}

impl Source for Static {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        out.write("extern ");
        if let Type::ConstPtr(..) = self.ty { } else {
            if !self.mutable {
                out.write("const ");
            }
        }
        self.ty.write(config, out);
        out.write(&format!(" {};", self.name));
    }
}
