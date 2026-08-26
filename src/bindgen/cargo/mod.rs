/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[allow(clippy::module_inception)]
mod cargo;
pub(crate) mod cargo_expand;
pub(crate) mod cargo_lock;
pub(crate) mod cargo_metadata;
pub(crate) mod cargo_toml;

pub(crate) use self::cargo::*;

use std::env;

/// Picks the cargo binary to invoke for `cargo metadata` and `cargo expand`.
///
/// An explicit choice (the `--cargo-bin` flag) wins, then the `CARGO`
/// environment variable that cargo itself sets when cbindgen runs as a
/// subcommand, and finally a bare `cargo` from `PATH`.
pub(crate) fn resolve_cargo_bin(explicit: Option<&str>) -> String {
    resolve_cargo_bin_from(explicit, env::var("CARGO").ok())
}

fn resolve_cargo_bin_from(explicit: Option<&str>, env_cargo: Option<String>) -> String {
    explicit
        .map(String::from)
        .or(env_cargo)
        .unwrap_or_else(|| String::from("cargo"))
}

#[cfg(test)]
mod tests {
    use super::resolve_cargo_bin_from;

    #[test]
    fn explicit_wins_over_env_and_default() {
        assert_eq!(
            resolve_cargo_bin_from(Some("/opt/cargo"), Some("/env/cargo".to_owned())),
            "/opt/cargo"
        );
        assert_eq!(
            resolve_cargo_bin_from(Some("/opt/cargo"), None),
            "/opt/cargo"
        );
    }

    #[test]
    fn env_used_when_no_explicit() {
        assert_eq!(
            resolve_cargo_bin_from(None, Some("/env/cargo".to_owned())),
            "/env/cargo"
        );
    }

    #[test]
    fn falls_back_to_bare_cargo() {
        assert_eq!(resolve_cargo_bin_from(None, None), "cargo");
    }
}
