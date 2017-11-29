/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::path::{Path, PathBuf};

use bindgen::cargo::cargo_expand;
use bindgen::cargo::cargo_lock::{self, Lock};
use bindgen::cargo::cargo_metadata::{self, Metadata};
use bindgen::cargo::cargo_toml;

/// Parse a dependency string used in Cargo.lock
fn parse_dep_string(dep_string: &str) -> (&str, &str) {
    let split: Vec<&str> = dep_string.split_whitespace().collect();

    (split[0], split[1])
}

/// A reference to a package including it's name and the specific version.
pub(crate) struct PackageRef {
    pub name: String,
    pub version: String,
}

/// A collection of metadata for a library from cargo.
#[derive(Clone, Debug)]
pub(crate) struct Cargo {
    manifest_path: PathBuf,
    binding_crate_name: String,

    lock: Option<Lock>,
    metadata: Metadata,
}

impl Cargo {
    /// Gather metadata from cargo for a specific library and binding crate
    /// name. If dependency finding isn't needed then Cargo.lock files don't
    /// need to be parsed.
    pub(crate) fn load(
        crate_dir: &Path,
        binding_crate_name: Option<&str>,
        use_cargo_lock: bool,
    ) -> Result<Cargo, String> {
        let toml_path = crate_dir.join("Cargo.toml");
        let lock_path = crate_dir.join("Cargo.lock");

        let lock = if use_cargo_lock {
            match cargo_lock::lock(&lock_path) {
                Ok(lock) => Some(lock),
                Err(x) => {
                    warn!("couldn't load lock file {:?}: {:?}", lock_path, x);
                    None
                }
            }
        } else {
            None
        };
        let metadata = cargo_metadata::metadata(&toml_path).map_err(|x| {
            format!(
                "couldn't execute `cargo metadata` with manifest {:?}: {:?}",
                toml_path,
                x
            )
        })?;

        // Use the specified binding crate name or infer it from the manifest
        let manifest = cargo_toml::manifest(&toml_path)
            .map_err(|_| format!("couldn't load {:?}.", toml_path))?;

        let binding_crate_name =
            binding_crate_name.map_or(manifest.package.name.clone(), |x| x.to_owned());

        Ok(Cargo {
            manifest_path: toml_path,
            binding_crate_name: binding_crate_name,
            lock: lock,
            metadata: metadata,
        })
    }

    pub(crate) fn binding_crate_name(&self) -> &str {
        &self.binding_crate_name
    }

    pub(crate) fn binding_crate_ref(&self) -> PackageRef {
        self.find_pkg_ref(&self.binding_crate_name).unwrap()
    }

    /// Finds the package reference in `cargo metadata` that has `package_name`
    /// ignoring the version.
    fn find_pkg_ref(&self, package_name: &str) -> Option<PackageRef> {
        for package in &self.metadata.packages {
            if package.name == package_name {
                return Some(PackageRef {
                    name: package_name.to_owned(),
                    version: package.version.clone(),
                });
            }
        }
        None
    }

    /// Finds the package reference for a dependency of a crate using
    /// `Cargo.lock`.
    pub(crate) fn find_dep_ref(
        &self,
        package: &PackageRef,
        dependency_name: &str,
    ) -> Option<PackageRef> {
        if self.lock.is_none() {
            return None;
        }
        let lock = self.lock.as_ref().unwrap();

        // the name in Cargo.lock could use '-' instead of '_', so we need to
        // look for that too.
        let replaced_name = dependency_name.replace("_", "-");

        if let &Some(ref root) = &lock.root {
            if root.name == package.name && root.version == package.version {
                if let Some(ref deps) = root.dependencies {
                    for dep in deps {
                        let (name, version) = parse_dep_string(dep);

                        if name == dependency_name || name == &replaced_name {
                            return Some(PackageRef {
                                name: name.to_owned(),
                                version: version.to_owned(),
                            });
                        }
                    }
                }
                return None;
            }
        }

        if let &Some(ref lock_packages) = &lock.package {
            for lock_package in lock_packages {
                if lock_package.name == package.name && lock_package.version == package.version {
                    if let Some(ref deps) = lock_package.dependencies {
                        for dep in deps {
                            let (name, version) = parse_dep_string(dep);

                            if name == dependency_name || name == &replaced_name {
                                return Some(PackageRef {
                                    name: name.to_owned(),
                                    version: version.to_owned(),
                                });
                            }
                        }
                    }
                    return None;
                }
            }
        }
        None
    }

    /// Finds the directory for a specified package reference.
    #[allow(unused)]
    pub(crate) fn find_crate_dir(&self, package: &PackageRef) -> Option<PathBuf> {
        for meta_package in &self.metadata.packages {
            if meta_package.name == package.name && meta_package.version == package.version {
                return Path::new(&meta_package.manifest_path)
                    .parent()
                    .map(|x| x.to_owned());
            }
        }
        None
    }

    /// Finds `src/lib.rs` for a specified package reference.
    pub(crate) fn find_crate_src(&self, package: &PackageRef) -> Option<PathBuf> {
        let kind_lib = String::from("lib");
        let kind_staticlib = String::from("staticlib");
        let kind_rlib = String::from("rlib");
        let kind_cdylib = String::from("cdylib");

        for meta_package in &self.metadata.packages {
            if meta_package.name == package.name && meta_package.version == package.version {
                for target in &meta_package.targets {
                    if target.kind.contains(&kind_lib) || target.kind.contains(&kind_staticlib)
                        || target.kind.contains(&kind_rlib)
                        || target.kind.contains(&kind_cdylib)
                    {
                        return Some(PathBuf::from(&target.src_path));
                    }
                }
                break;
            }
        }
        None
    }

    pub(crate) fn expand_crate(&self, package: &PackageRef) -> Result<String, String> {
        cargo_expand::expand(&self.manifest_path, &package.name, &package.version)
    }
}
