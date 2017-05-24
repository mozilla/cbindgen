#![deny(missing_docs)]
//! Structured access to the output of `cargo metadata`
//! Usually used from within a `cargo-*` executable
//!
//! ```rust
//! # extern crate cargo_metadata;
//! let manifest_path_arg = std::env::args().skip(2).find(|val| val.starts_with("--manifest-path="));
//! let metadata = cargo_metadata::metadata(manifest_path_arg.as_ref().map(AsRef::as_ref)).unwrap();
//! ```

// Forked from `https://github.com/oli-obk/cargo_metadata`
// Modifications:
//   1. Remove `resolve` from Metadata because it was causing parse failures
//   2. Fix the `manifest-path` argument
//   3. Add `--all-features` argument
//   4. Remove the `--no-deps` argument

use std::collections::HashMap;
use std::env;
use std::process::Command;
use std::str::{from_utf8, Utf8Error};
use std::io;

use serde_json;

#[derive(Clone, Deserialize, Debug)]
/// Starting point for metadata returned by `cargo metadata`
pub struct Metadata {
    /// A list of all crates referenced by this crate (and the crate itself)
    pub packages: Vec<Package>,
    version: usize,
}

#[derive(Clone, Deserialize, Debug)]
/// A crate
pub struct Package {
    /// Name as given in the `Cargo.toml`
    pub name: String,
    /// Version given in the `Cargo.toml`
    pub version: String,
    id: String,
    source: Option<String>,
    /// List of dependencies of this particular package
    pub dependencies: Vec<Dependency>,
    /// Targets provided by the crate (lib, bin, example, test, ...)
    pub targets: Vec<Target>,
    features: HashMap<String, Vec<String>>,
    /// path containing the `Cargo.toml`
    pub manifest_path: String,
}

#[derive(Clone, Deserialize, Debug)]
/// A dependency of the main crate
pub struct Dependency {
    /// Name as given in the `Cargo.toml`
    pub name: String,
    source: Option<String>,
    /// Whether this is required or optional
    pub req: String,
    kind: Option<String>,
    optional: bool,
    uses_default_features: bool,
    features: Vec<String>,
    target: Option<String>,
}

#[derive(Clone, Deserialize, Debug)]
/// A single target (lib, bin, example, ...) provided by a crate
pub struct Target {
    /// Name as given in the `Cargo.toml` or generated from the file name
    pub name: String,
    /// Kind of target ("bin", "example", "test", "bench", "lib")
    pub kind: Vec<String>,
    /// Almost the same as `kind`, except when an example is a library instad of an executable.
    /// In that case `crate_types` contains things like `rlib` and `dylib` while `kind` is `example`
    #[serde(default)]
    pub crate_types: Vec<String>,
    /// Path to the main source file of the target
    pub src_path: String,
}

#[derive(Debug)]
/// Possible errors that can occur during metadata parsing.
pub enum Error {
    /// Error during execution of `cargo metadata`
    Io(io::Error),
    /// Output of `cargo metadata` was not valid utf8
    Utf8(Utf8Error),
    /// Deserialization error (structure of json did not match expected structure)
    Json(serde_json::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}
impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error::Utf8(err)
    }
}
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

/// The main entry point to obtaining metadata
pub fn metadata(manifest_path_arg: Option<&str>) -> Result<Metadata, Error> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| String::from("cargo"));
    let mut cmd = Command::new(cargo);
    cmd.arg("metadata");
    cmd.arg("--all-features");
    cmd.arg("--format-version").arg("1");
    if let Some(mani) = manifest_path_arg {
        cmd.arg("--manifest-path");
        cmd.arg(mani);
    }
    let output = cmd.output()?;
    let stdout = from_utf8(&output.stdout)?;
    let meta: Metadata = serde_json::from_str(stdout)?;
    Ok(meta)
}
