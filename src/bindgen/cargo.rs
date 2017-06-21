/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::path::{Path, PathBuf};

use bindgen::cargo_expand;
use bindgen::cargo_lock::{self, Lock};
use bindgen::cargo_metadata::{self, Metadata};
use bindgen::cargo_toml;

pub struct Cargo {
    manifest_path: PathBuf,
    binding_crate_name: String,

    lock: Lock,
    metadata: Metadata,
}

impl Cargo {
    pub fn load(crate_dir: &Path, binding_crate_name: Option<&str>) -> Result<Cargo, String> {
        let toml_path = crate_dir.join("Cargo.toml");
        let lock_path = crate_dir.join("Cargo.lock");

        let lock = cargo_lock::lock(&lock_path)
                              .map_err(|x| format!("couldn't load {:?}: {:?}", lock_path, x))?;
        let metadata = cargo_metadata::metadata(&toml_path)
                                      .map_err(|_| format!("couldn't execute `cargo metadata` with manifest {:?}.", toml_path))?;

        // Use the specified binding crate name or infer it from the manifest
        let manifest = cargo_toml::manifest(&toml_path)
                                  .map_err(|_| format!("couldn't load {:?}.", toml_path))?;

        let binding_crate_name = binding_crate_name.map_or(manifest.package.name.clone(),
                                                           |x| x.to_owned());

        Ok(Cargo {
            manifest_path: toml_path,
            binding_crate_name: binding_crate_name,
            lock: lock,
            metadata: metadata,
        })
    }

    pub fn binding_crate_name(&self) -> &str {
        &self.binding_crate_name
    }

    pub fn find_crate_dir(&self, package_name: &str) -> Option<PathBuf> {
        for package in &self.metadata.packages {
            if package.name == package_name {
                return Path::new(&package.manifest_path).parent()
                                                        .map(|x| x.to_owned());
            }
        }
        None
    }

    pub fn find_crate_src(&self, package_name: &str) -> Option<PathBuf> {
        let kind_lib = String::from("lib");
        for package in &self.metadata.packages {
            if package.name == package_name {
                for target in &package.targets {
                    if target.kind.contains(&kind_lib) {
                        return Some(PathBuf::from(&target.src_path));
                    }
                }
                break;
            }
        }
        None
    }

    pub fn expand_crate(&self, crate_name: &str) -> Result<String, String> {
        cargo_expand::expand(&self.manifest_path, crate_name)
    }
}
