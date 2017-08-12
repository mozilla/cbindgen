/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate syn;
extern crate toml;

mod bindgen;

pub use bindgen::*;

use std::path::Path;

/// A utility function for build scripts to generate bindings for a crate, using
/// a `cbindgen.toml` if it exists.
pub fn generate(crate_dir: &str) -> Result<Bindings, String> {
    let config = Config::from_root_or_default(Path::new(crate_dir));

    generate_with_config(crate_dir, config)
}

/// A utility function for build scripts to generate bindings for a crate with a
/// custom config.
pub fn generate_with_config(crate_dir: &str, config: Config) -> Result<Bindings, String> {
    let crate_dir = Path::new(crate_dir);
    let cargo = Cargo::load(crate_dir,
                            None,
                            config.parse.parse_deps)?;

    LibraryBuilder::new().with_config(config)
                         .with_std_types()
                         .with_crate(cargo)
                         .build()?.generate()
}
