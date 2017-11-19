/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/// A helper macro for deriving deserialize for an enum to be used in toml-rs.
/// This macro works be relying on an existing FromStr implementation for the
/// desired type.
macro_rules! deserialize_enum_str {
    ($name:ident) => {
        impl ::serde::Deserialize for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer
            {
                struct Visitor;
                impl ::serde::de::Visitor for Visitor {
                    type Value = $name;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        f.write_str("$name")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<$name, E>
                        where E: ::serde::de::Error
                    {
                        match v.parse::<$name>() {
                            Ok(v) => Ok(v),
                            Err(m) => Err(E::custom(m)),
                        }
                    }
                }
                deserializer.deserialize_str(Visitor)
            }
        }
    }
}

mod bindings;
mod builder;
mod cargo;
mod cdecl;
mod config;
mod dependencies;
mod ir;
mod library;
mod mangle;
mod monomorph;
mod parser;
mod rename;
mod utilities;
mod writer;

#[allow(unused)]
pub(crate) use self::cargo::*;

pub use self::config::*;
pub use self::bindings::Bindings;
pub use self::builder::Builder;
