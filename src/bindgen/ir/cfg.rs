/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use syn;

use bindgen::config::Config;
use bindgen::writer::SourceWriter;

enum DefineKey<'a> {
    Boolean(&'a str),
    Named(&'a str, &'a str),
}

impl<'a> DefineKey<'a> {
    fn load(key: &str) -> DefineKey {
        // TODO: dirty parser
        if key.contains("=") {
            let mut splits = key.trim().split("=");

            let name = if let Some(name) = splits.next() {
                name.trim()
            } else {
                return DefineKey::Boolean(key);
            };

            let value = if let Some(value) = splits.next() {
                value.trim()
            } else {
                return DefineKey::Boolean(key);
            };

            if splits.next().is_some() {
                return DefineKey::Boolean(key);
            }

            DefineKey::Named(name, value)
        } else {
            return DefineKey::Boolean(key);
        }
    }
}

#[derive(Debug, Clone)]
pub enum Cfg {
    Boolean(String),
    Named(String, String),
    Any(Vec<Cfg>),
    All(Vec<Cfg>),
    Not(Box<Cfg>),
}

impl Cfg {
    pub fn join(cfgs: &[Cfg]) -> Option<Cfg> {
        if cfgs.len() == 0 {
            None
        } else {
            Some(Cfg::All(cfgs.to_owned()))
        }
    }

    pub fn append(parent: &Option<Cfg>, child: Option<Cfg>) -> Option<Cfg> {
        match (parent, child) {
            (&None, None) => None,
            (&None, Some(child)) => Some(child),
            (&Some(ref parent), None) => Some(parent.clone()),
            (&Some(ref parent), Some(ref child)) => {
                Some(Cfg::All(vec![parent.clone(), child.clone()]))
            }
        }
    }

    pub fn load(attrs: &[syn::Attribute]) -> Option<Cfg> {
        let mut configs = Vec::new();

        for attr in attrs {
            if attr.is_sugared_doc {
                continue;
            }

            match attr.interpret_meta() {
                Some(syn::Meta::List(syn::MetaList { ident, nested, .. })) => {
                    if ident != "cfg" || nested.len() != 1 {
                        continue;
                    }

                    if let Some(config) = Cfg::load_single(nested.first().unwrap().value()) {
                        configs.push(config);
                    }
                }
                _ => {}
            }
        }

        match configs.len() {
            0 => None,
            1 => Some(configs.pop().unwrap()),
            _ => Some(Cfg::All(configs)),
        }
    }

    fn load_single(item: &syn::NestedMeta) -> Option<Cfg> {
        match item {
            &syn::NestedMeta::Meta(syn::Meta::Word(ref ident)) => {
                Some(Cfg::Boolean(format!("{}", ident)))
            }
            &syn::NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue {
                ref ident,
                ref lit,
                ..
            })) => match lit {
                &syn::Lit::Str(ref value) => Some(Cfg::Named(format!("{}", ident), value.value())),
                _ => None,
            },
            &syn::NestedMeta::Meta(syn::Meta::List(syn::MetaList {
                ref ident,
                ref nested,
                ..
            })) => {
                if ident == "any" {
                    if let Some(configs) = Cfg::load_list(nested.iter()) {
                        Some(Cfg::Any(configs))
                    } else {
                        None
                    }
                } else if ident == "all" {
                    if let Some(configs) = Cfg::load_list(nested.iter()) {
                        Some(Cfg::All(configs))
                    } else {
                        None
                    }
                } else if ident == "not" {
                    if nested.len() != 1 {
                        return None;
                    }

                    if let Some(config) = Cfg::load_single(&nested[0]) {
                        Some(Cfg::Not(Box::new(config)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn load_list<'a, I: Iterator<Item = &'a syn::NestedMeta>>(attrs: I) -> Option<Vec<Cfg>> {
        let mut configs = Vec::new();

        for attr in attrs {
            if let Some(config) = Cfg::load_single(attr) {
                configs.push(config);
            } else {
                return None;
            }
        }

        if configs.is_empty() {
            None
        } else {
            Some(configs)
        }
    }

    fn has_defines(&self, config: &Config) -> bool {
        match self {
            &Cfg::Boolean(ref cfg_name) => {
                for (key, ..) in &config.defines {
                    let key = DefineKey::load(key);

                    match key {
                        DefineKey::Boolean(key_name) => if cfg_name == key_name {
                            return true;
                        },
                        DefineKey::Named(..) => {}
                    }
                }

                false
            }
            &Cfg::Named(ref cfg_name, ref cfg_value) => {
                for (key, ..) in &config.defines {
                    let key = DefineKey::load(key);

                    match key {
                        DefineKey::Boolean(..) => {}
                        DefineKey::Named(key_name, key_value) => {
                            if cfg_name == key_name && cfg_value == key_value {
                                return true;
                            }
                        }
                    }
                }

                false
            }
            &Cfg::Any(ref cfgs) => cfgs.iter().all(|x| x.has_defines(config)),
            &Cfg::All(ref cfgs) => cfgs.iter().all(|x| x.has_defines(config)),
            &Cfg::Not(ref cfg) => cfg.has_defines(config),
        }
    }

    fn write_condition<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        match self {
            &Cfg::Boolean(ref cfg_name) => {
                let mut define: &str = cfg_name;

                for (key, define_value) in &config.defines {
                    let key = DefineKey::load(key);

                    match key {
                        DefineKey::Boolean(key_name) => if cfg_name == key_name {
                            define = define_value;
                        },
                        DefineKey::Named(..) => {}
                    }
                }

                out.write("defined(");
                write!(out, "{}", define);
                out.write(")");
            }
            &Cfg::Named(ref cfg_name, ref cfg_value) => {
                let mut define: &str = cfg_name;

                for (key, define_value) in &config.defines {
                    let key = DefineKey::load(key);

                    match key {
                        DefineKey::Boolean(..) => {}
                        DefineKey::Named(key_name, key_value) => {
                            if cfg_name == key_name && cfg_value == key_value {
                                define = define_value;
                            }
                        }
                    }
                }

                out.write("defined(");
                write!(out, "{}", define);
                out.write(")");
            }
            &Cfg::Any(ref cfgs) => {
                out.write("(");
                for (i, cfg) in cfgs.iter().enumerate() {
                    if i != 0 {
                        out.write(" || ");
                    }
                    cfg.write_condition(config, out);
                }
                out.write(")");
            }
            &Cfg::All(ref cfgs) => {
                out.write("(");
                for (i, cfg) in cfgs.iter().enumerate() {
                    if i != 0 {
                        out.write(" && ");
                    }
                    cfg.write_condition(config, out);
                }
                out.write(")");
            }
            &Cfg::Not(ref cfg) => {
                out.write("!");
                cfg.write_condition(config, out);
            }
        }
    }
}

pub trait CfgWrite {
    fn write_before<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>);

    fn write_after<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>);
}

impl CfgWrite for Option<Cfg> {
    fn write_before<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        if let &Some(ref cfg) = self {
            if !cfg.has_defines(config) {
                return;
            }

            out.write("#if ");
            cfg.write_condition(config, out);
            out.new_line();
        }
    }

    fn write_after<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        if let &Some(ref cfg) = self {
            // TODO
            if !cfg.has_defines(config) {
                return;
            }

            out.new_line();
            out.write("#endif");
        }
    }
}
