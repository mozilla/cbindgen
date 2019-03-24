/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[macro_use]
extern crate log;
extern crate proc_macro2;
#[macro_use]
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate toml;

mod bindgen;

pub use bindgen::*;

use std::path::Path;

/// A utility function for build scripts to generate bindings for a crate, using
/// a `cbindgen.toml` if it exists.
pub fn generate<P: AsRef<Path>>(crate_dir: P) -> Result<Bindings, Error> {
    let config = ConfigLoader::from_root_or_default(crate_dir.as_ref());
    let root = config.root().clone();

    generate_with_config(crate_dir.as_ref(), &root, config.into())
}

/// A utility function for build scripts to generate bindings for a crate with a
/// custom config.
pub fn generate_with_config<P: AsRef<Path>>(
    crate_dir: P,
    config_root: P,
    config: Config,
) -> Result<Bindings, Error> {
    Builder::new()
        .with_config(config)
        .with_root(config_root)
        .with_crate(crate_dir)
        .generate()
}
