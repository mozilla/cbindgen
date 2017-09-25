/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use bindgen::ir::{AnnotationSet, Cfg, Documentation, Item, Type};
use bindgen::library::Library;
use bindgen::writer::{Source, SourceWriter};
use bindgen::dependencies::DependencyKind;
use bindgen::mangle;
use bindgen::config::Config;
use std::io::Write;

/// A type alias that generates a copy of its aliasee with a new name. If the type
/// alias has generic values, it specializes its aliasee. This is useful for
/// presenting an interface that includes generic types without mangling.
#[derive(Debug, Clone)]
pub struct Specialization {
    pub name: String,
    pub generic_params: Vec<String>,
    pub generic_values: Vec<Type>,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}

impl Specialization {

    pub fn get_deps(&self, library: &Library) -> Vec<(Item, DependencyKind)> {
        if self.generic_values.is_empty() {
            return Vec::new();
        }
        if let Some(v) = library.get_item(&self.name) {
            let mut ret = self.generic_values.iter()
                .flat_map(|g| g.get_items(library, DependencyKind::Normal))
                .collect::<Vec<_>>();
            match v {
                Item::Struct(mut s) => {
                    s.name = mangle::mangle_path(&s.name, &self.generic_values);
                    ret.push((Item::Specialization(Specialization {
                        generic_values: Vec::new(),
                        ..self.clone()
                    }), DependencyKind::Normal));
                    ret
                }
                Item::Typedef(mut t) => {
                    t.name = mangle::mangle_path(&t.name, &self.generic_values);
                    ret.push((Item::Specialization(Specialization {
                        generic_values: Vec::new(),
                        ..self.clone()
                    }), DependencyKind::Normal));
                    ret
                }
                e =>{ println!("{:?}", e); unimplemented!()}
            }
        } else {
            Vec::new()
        }
    }
}

impl Source for Specialization {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        if !config.structure.generic_template_specialization {
            return;
        }
        self.documentation.write(config, out);
        if self.generic_values.is_empty() {
            out.write("template<");
            let mut first = true;
            for t in &self.generic_params {
                if first {
                    first = false;
                } else {
                    out.write(", ");
                }
                out.write(&format!("typename {}", t));
            }
            out.write(">");
            out.new_line();
            out.write(&format!("struct {};", self.name));
        } else {
            out.write("template<>");
            out.new_line();
            out.write(&format!("struct {}<", self.name));
            let mut first = true;
            for t in &self.generic_values {
                if first {
                    first = false;
                } else {
                    out.write(", ");
                }
                t.write(config, out);
            }
            out.write(&format!("> : public {}", mangle::mangle_path(&self.name, &self.generic_values)));
            out.open_brace();
            out.close_brace(true);
        }
    }
}
