/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::env;
use std::io;
use std::path::{Path, PathBuf};

extern crate clap;
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

use clap::builder::PossibleValuesParser;
use clap::ArgAction;
use clap::{Arg, ArgMatches, Command};

mod bindgen;
mod logging;

use crate::bindgen::{Bindings, Builder, Cargo, Config, Error};

fn apply_config_overrides(config: &mut Config, matches: &ArgMatches) {
    // We allow specifying a language to override the config default. This is
    // used by compile-tests.
    match matches.try_get_one("lang") {
        Ok(Some(lang)) => {
            config.language = *lang;
        }
        Err(reason) => {
            error!("{}", reason);
            return;
        }
        _ => (),
    }

    if matches.contains_id("cpp-compat") {
        config.cpp_compat = true;
    }

    if matches.contains_id("only-target-dependencies") {
        config.only_target_dependencies = true;
    }

    match matches.try_get_one("style") {
        Ok(Some(style)) => {
            config.style = *style;
        }
        Err(_) => {
            error!("Unknown style specified.");
            return;
        }
        _ => (),
    }

    match matches.try_get_one("profile") {
        Ok(Some(profile)) => {
            config.parse.expand.profile = *profile;
        }
        Err(e) => {
            error!("{}", e);
            return;
        }
        _ => (),
    }

    if matches.contains_id("d") {
        config.parse.parse_deps = true;
    }
}

fn load_bindings(input: &Path, matches: &ArgMatches) -> Result<Bindings, Error> {
    // If a file is specified then we load it as a single source
    if !input.is_dir() {
        // Load any config specified or search in the input directory
        let mut config = match matches.get_one::<PathBuf>("config") {
            Some(c) => Config::from_file(c).unwrap(),
            None => Config::from_root_or_default(
                input
                    .parent()
                    .expect("All files should have a parent directory"),
            ),
        };

        apply_config_overrides(&mut config, matches);

        return Builder::new()
            .with_config(config)
            .with_src(input)
            .generate();
    }

    // We have to load a whole crate, so we use cargo to gather metadata
    let lib = Cargo::load(
        input,
        matches.get_one::<String>("lockfile").map(|s| s.as_str()),
        matches.get_one::<String>("crate").map(|s| s.as_str()),
        true,
        matches.contains_id("clean"),
        matches.contains_id("only-target-dependencies"),
        matches.get_one::<PathBuf>("metadata").map(|p| p.as_path()),
    )?;

    // Load any config specified or search in the binding crate directory
    let mut config = match matches.get_one::<PathBuf>("config") {
        Some(c) => Config::from_file(c).unwrap(),
        None => {
            let binding_crate_dir = lib.find_crate_dir(&lib.binding_crate_ref());

            if let Some(binding_crate_dir) = binding_crate_dir {
                Config::from_root_or_default(binding_crate_dir)
            } else {
                // This shouldn't happen
                Config::from_root_or_default(input)
            }
        }
    };

    apply_config_overrides(&mut config, matches);

    Builder::new()
        .with_config(config)
        .with_cargo(lib)
        .generate()
}

fn main() {
    let matches = Command::new("cbindgen")
        .version(bindgen::VERSION)
        .about("Generate C bindings for a Rust library")
        .arg(
            Arg::new("v")
                .short('v')
                .action(ArgAction::Count)
                .help("Enable verbose logging"),
        )
        .arg(
            Arg::new("verify")
                .long("verify")
                .help("Generate bindings and compare it to the existing bindings file and error if they are different"),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("PATH")
                .help("Specify path to a `cbindgen.toml` config to use"),
        )
        .arg(
            Arg::new("lang")
                .short('l')
                .long("lang")
                .value_name("LANGUAGE")
                .help("Specify the language to output bindings in")
                .value_parser(PossibleValuesParser::new(["c++", "C++", "c", "C", "cython", "Cython"])),
        )
        .arg(
            Arg::new("cpp-compat")
                .long("cpp-compat")
                .help("Whether to add C++ compatibility to generated C bindings")
        )
        .arg(
            Arg::new("only-target-dependencies")
                .long("only-target-dependencies")
                .help("Only fetch dependencies needed by the target platform. \
                    The target platform defaults to the host platform; set TARGET to override.")
        )
        .arg(
            Arg::new("style")
                .short('s')
                .long("style")
                .value_name("STYLE")
                .help("Specify the declaration style to use for bindings")
                .value_parser(PossibleValuesParser::new(["Both", "both", "Tag", "tag", "Type", "type"])),
        )
        .arg(
            Arg::new("d")
                .short('d')
                .long("parse-dependencies")
                .help("Whether to parse dependencies when generating bindings"),
        )
        .arg(
            Arg::new("clean")
                .long("clean")
                .help(
                    "Whether to use a new temporary directory for expanding macros. \
                    Affects performance, but might be required in certain build processes.")
                .required(false)
        )
        .arg(
            Arg::new("INPUT")
                .help(
                    "A crate directory or source file to generate bindings for. \
                    In general this is the folder where the Cargo.toml file of \
                    source Rust library resides.")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("crate")
                .long("crate")
                .value_name("CRATE_NAME")
                .help(
                    "If generating bindings for a crate, \
                     the specific crate to generate bindings for",
                )
                .required(false),
        )
        .arg(
            Arg::new("out")
                .short('o')
                .long("output")
                .value_name("PATH")
                .help("The file to output the bindings to")
                .required(false),
        )
        .arg(
            Arg::new("lockfile")
                .long("lockfile")
                .value_name("PATH")
                .help(
                    "Specify the path to the Cargo.lock file explicitly. If this \
                    is not specified, the Cargo.lock file is searched for in the \
                    same folder as the Cargo.toml file. This option is useful for \
                    projects that use workspaces.")
                .required(false),
        )
        .arg(
            Arg::new("metadata")
                .long("metadata")
                .value_name("PATH")
                .help(
                    "Specify the path to the output of a `cargo metadata` \
                     command that allows to get dependency information. \
                     This is useful because cargo metadata may be the longest \
                     part of cbindgen runtime, and you may want to share it \
                     across cbindgen invocations. By default cbindgen will run \
                     `cargo metadata --all-features --format-version 1 \
                      --manifest-path <path/to/crate/Cargo.toml>"
                )
                .required(false),
        )
        .arg(
            Arg::new("profile")
                .long("profile")
                .value_name("PROFILE")
                .help(
                    "Specify the profile to use when expanding macros. \
                     Has no effect otherwise."
                )
                .value_parser(PossibleValuesParser::new(["Debug", "debug", "Release", "release"])),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Report errors only (overrides verbosity options).")
                .required(false),
        )
        .arg(
            Arg::new("depfile")
                .value_name("PATH")
                .long("depfile")
                .num_args(1)
                .required(false)
                .help("Generate a depfile at the given Path listing the source files \
                    cbindgen traversed when generating the bindings. Useful when \
                    integrating cbindgen into 3rd party build-systems. \
                    This option is ignored if `--out` is missing."
                )
        )
        .get_matches();

    if !matches.contains_id("out") && matches.contains_id("verify") {
        error!(
            "Cannot verify bindings against `stdout`, please specify a file to compare against."
        );
        std::process::exit(2);
    }

    // Initialize logging
    if matches.contains_id("quiet") {
        logging::ErrorLogger::init().unwrap();
    } else {
        match matches.get_count("v") {
            0 => logging::WarnLogger::init().unwrap(),
            1 => logging::InfoLogger::init().unwrap(),
            _ => logging::TraceLogger::init().unwrap(),
        }
    }

    // Find the input directory
    let input: PathBuf = matches
        .get_one("INPUT")
        .cloned()
        .unwrap_or_else(|| env::current_dir().unwrap());

    let bindings = match load_bindings(&input, &matches) {
        Ok(bindings) => bindings,
        Err(msg) => {
            error!("{}", msg);
            error!("Couldn't generate bindings for {}.", input.display());
            std::process::exit(1);
        }
    };

    // Write the bindings file
    match matches.get_one::<String>("out") {
        Some(file) => {
            let changed = bindings.write_to_file(file);

            if matches.contains_id("verify") && changed {
                error!("Bindings changed: {}", file);
                std::process::exit(2);
            }
            if let Some(depfile) = matches.get_one("depfile") {
                bindings.generate_depfile(file, depfile)
            }
        }
        _ => {
            bindings.write(io::stdout());
        }
    }
}
