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

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        formatter.write_str("$name")
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

mod cargo;
mod cargo_expand;
mod cargo_lock;
mod cargo_metadata;
mod cargo_toml;
mod cdecl;
mod config;
mod annotation;
mod items;
mod library;
mod rename;
mod rust_lib;
mod utilities;
mod writer;

pub use self::cargo::*;
pub use self::config::*;
pub use self::library::Library;
pub use self::cargo_toml::manifest;
