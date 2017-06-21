/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::env;
use std::path::Path;
use std::process::Command;
use std::str::from_utf8;

/// Use rustc to expand and pretty print the crate into a single file,
/// removing any macros in the process.
pub fn expand(manifest_path: &Path,
              crate_name: &str,
              version: &str) -> Result<String, String> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| String::from("cargo"));

    let mut cmd = Command::new(cargo);
    cmd.arg("rustc");
    cmd.arg("--manifest-path");
    cmd.arg(manifest_path);
    cmd.arg("--all-features");
    cmd.arg("-p");
    cmd.arg(&format!("{}:{}", crate_name, version));
    cmd.arg("--");
    cmd.arg("-Z");
    cmd.arg("unstable-options");
    cmd.arg("--pretty=expanded");
    let output = cmd.output().unwrap();

    let src = from_utf8(&output.stdout).unwrap().to_owned();
    let error = from_utf8(&output.stderr).unwrap().to_owned();

    if src.len() == 0 {
        Err(error)
    } else {
        Ok(src)
    }
}
