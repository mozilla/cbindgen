/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::fmt;
use std::io::Write;

use syn;

use bindgen::cargo::cargo_metadata::Dependency;
use bindgen::config::Config;
use bindgen::writer::SourceWriter;

#[derive(PartialEq, Eq)]
enum DefineKey<'a> {
    Boolean(&'a str),
    Named(&'a str, &'a str),
}

impl<'a> DefineKey<'a> {
    fn load(key: &str) -> DefineKey {
        // TODO: dirty parser
        if key.contains('=') {
            let mut splits = key.trim().split('=');

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
            DefineKey::Boolean(key)
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

impl fmt::Display for Cfg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cfg::Boolean(key) => write!(f, "{}", key),
            Cfg::Named(key, value) => write!(f, "{} = {:?}", key, value),
            Cfg::Any(cfgs) => {
                write!(f, "any(")?;
                for (index, cfg) in cfgs.iter().enumerate() {
                    if index > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", cfg)?;
                }
                write!(f, ")")
            }
            Cfg::All(cfgs) => {
                write!(f, "all(")?;
                for (index, cfg) in cfgs.iter().enumerate() {
                    if index > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", cfg)?;
                }
                write!(f, ")")
            }
            Cfg::Not(cfg) => write!(f, "not({})", cfg),
        }
    }
}

impl Cfg {
    pub fn join(cfgs: &[Cfg]) -> Option<Cfg> {
        if cfgs.is_empty() {
            None
        } else {
            Some(Cfg::All(cfgs.to_owned()))
        }
    }

    pub fn append(parent: Option<&Cfg>, child: Option<Cfg>) -> Option<Cfg> {
        match (parent, child) {
            (None, None) => None,
            (None, Some(child)) => Some(child),
            (Some(parent), None) => Some(parent.clone()),
            (Some(parent), Some(child)) => Some(Cfg::All(vec![parent.clone(), child])),
        }
    }

    pub fn load(attrs: &[syn::Attribute]) -> Option<Cfg> {
        let mut configs = Vec::new();

        for attr in attrs {
            if let Ok(syn::Meta::List(syn::MetaList { path, nested, .. })) = attr.parse_meta() {
                if !path.is_ident("cfg") || nested.len() != 1 {
                    continue;
                }

                if let Some(config) = Cfg::load_single(nested.first().unwrap()) {
                    configs.push(config);
                }
            }
        }

        match configs.len() {
            0 => None,
            1 => Some(configs.pop().unwrap()),
            _ => Some(Cfg::All(configs)),
        }
    }

    pub fn load_metadata(dependency: &Dependency) -> Option<Cfg> {
        dependency
            .target
            .as_ref()
            .map(|target| {
                syn::parse_str::<syn::Meta>(target)
                    .expect("error parsing dependency's target metadata")
            })
            .and_then(|target| {
                if let syn::Meta::List(syn::MetaList { path, nested, .. }) = target {
                    if !path.is_ident("cfg") || nested.len() != 1 {
                        return None;
                    }
                    Cfg::load_single(nested.first().unwrap())
                } else {
                    None
                }
            })
    }

    fn load_single(item: &syn::NestedMeta) -> Option<Cfg> {
        match *item {
            syn::NestedMeta::Meta(syn::Meta::Path(ref path)) => Some(Cfg::Boolean(format!(
                "{}",
                path.segments.first().unwrap().ident
            ))),
            syn::NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue {
                ref path,
                ref lit,
                ..
            })) => match lit {
                &syn::Lit::Str(ref value) => Some(Cfg::Named(
                    format!("{}", path.segments.first().unwrap().ident),
                    value.value(),
                )),
                _ => None,
            },
            syn::NestedMeta::Meta(syn::Meta::List(syn::MetaList {
                ref path,
                ref nested,
                ..
            })) => {
                if path.is_ident("any") {
                    if let Some(configs) = Cfg::load_list(nested.iter()) {
                        Some(Cfg::Any(configs))
                    } else {
                        None
                    }
                } else if path.is_ident("all") {
                    if let Some(configs) = Cfg::load_list(nested.iter()) {
                        Some(Cfg::All(configs))
                    } else {
                        None
                    }
                } else if path.is_ident("not") {
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
}

pub trait ToCondition: Sized {
    type Output;

    fn to_condition(self, config: &Config) -> Option<Self::Output>;
}

impl<'a> ToCondition for &'a Option<Cfg> {
    type Output = Condition;

    fn to_condition(self, config: &Config) -> Option<Self::Output> {
        self.clone().and_then(|cfg| cfg.to_condition(config))
    }
}

impl ToCondition for Option<Cfg> {
    type Output = Condition;

    fn to_condition(self, config: &Config) -> Option<Self::Output> {
        self.and_then(|cfg| cfg.to_condition(config))
    }
}

impl<'a> ToCondition for &'a Cfg {
    type Output = Condition;

    fn to_condition(self, config: &Config) -> Option<Self::Output> {
        self.clone().to_condition(config)
    }
}

impl ToCondition for Cfg {
    type Output = Condition;

    fn to_condition(self, config: &Config) -> Option<Self::Output> {
        match self {
            Cfg::Boolean(cfg_name) => {
                let define = config
                    .defines
                    .iter()
                    .find(|(key, ..)| DefineKey::Boolean(&cfg_name) == DefineKey::load(key));
                if let Some((_, define)) = define {
                    Some(Condition::Define(define.to_owned()))
                } else {
                    warn!(
                        "Missing `[defines]` entry for `{}` in cbindgen config.",
                        Cfg::Boolean(cfg_name)
                    );
                    None
                }
            }
            Cfg::Named(cfg_name, cfg_value) => {
                let define = config.defines.iter().find(|(key, ..)| {
                    DefineKey::Named(&cfg_name, &cfg_value) == DefineKey::load(key)
                });
                if let Some((_, define)) = define {
                    Some(Condition::Define(define.to_owned()))
                } else {
                    warn!(
                        "Missing `[defines]` entry for `{}` in cbindgen config.",
                        Cfg::Named(cfg_name, cfg_value)
                    );
                    None
                }
            }
            Cfg::Any(children) => {
                let conditions: Vec<_> = children
                    .into_iter()
                    .filter_map(|x| x.to_condition(config))
                    .collect();
                match conditions.len() {
                    0 => None,
                    1 => conditions.into_iter().next(),
                    _ => Some(Condition::Any(conditions)),
                }
            }
            Cfg::All(children) => {
                let cfgs: Vec<_> = children
                    .into_iter()
                    .filter_map(|x| x.to_condition(config))
                    .collect();
                match cfgs.len() {
                    0 => None,
                    1 => cfgs.into_iter().next(),
                    _ => Some(Condition::All(cfgs)),
                }
            }
            Cfg::Not(child) => child
                .to_condition(config)
                .map(|cfg| Condition::Not(Box::new(cfg))),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Condition {
    Define(String),
    Any(Vec<Condition>),
    All(Vec<Condition>),
    Not(Box<Condition>),
}

impl Condition {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        match *self {
            Condition::Define(ref define) => {
                out.write("defined(");
                write!(out, "{}", define);
                out.write(")");
            }
            Condition::Any(ref conditions) => {
                out.write("(");
                for (i, condition) in conditions.iter().enumerate() {
                    if i != 0 {
                        out.write(" || ");
                    }
                    condition.write(config, out);
                }
                out.write(")");
            }
            Condition::All(ref conditions) => {
                out.write("(");
                for (i, condition) in conditions.iter().enumerate() {
                    if i != 0 {
                        out.write(" && ");
                    }
                    condition.write(config, out);
                }
                out.write(")");
            }
            Condition::Not(ref condition) => {
                out.write("!");
                condition.write(config, out);
            }
        }
    }
}

pub trait ConditionWrite {
    fn write_before<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>);
    fn write_after<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>);
}

impl ConditionWrite for Option<Condition> {
    fn write_before<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        if let Some(ref cfg) = *self {
            out.write("#if ");
            cfg.write(config, out);
            out.new_line();
        }
    }

    fn write_after<F: Write>(&self, _config: &Config, out: &mut SourceWriter<F>) {
        if self.is_some() {
            out.new_line();
            out.write("#endif");
        }
    }
}
