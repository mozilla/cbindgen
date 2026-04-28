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

use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};

mod bindgen;
mod logging;

use crate::bindgen::{Bindings, Builder, Cargo, Error};

fn load_bindings(input: &Path, matches: &ArgMatches) -> Result<Bindings, Error> {
    // We have to load a whole crate, so we use cargo to gather metadata
    let lib = Cargo::load(
        input,
        matches.get_one::<PathBuf>("lockfile").map(|s| s.as_path()),
        matches.get_one::<String>("crate").map(|s| s.as_str()),
        true,
        matches.get_flag("clean"),
        matches.get_flag("only-target-dependencies"),
        matches.get_one::<PathBuf>("metadata").map(|p| p.as_path()),
    )?;

    Builder::new().with_gobject(true).with_cargo(lib).generate()
}

fn main() {
    let matches = Command::new("gbindgen")
        .version(bindgen::VERSION)
        .about("Generate GObject C bindings for a glib/gtk-rs library")
        .arg(
            Arg::new("v")
                .short('v')
                .action(ArgAction::Count)
                .help("Enable verbose logging"),
        )
        .arg(
            Arg::new("verify")
                .long("verify")
                .action(ArgAction::SetTrue)
                .help("Generate bindings and compare it to the existing bindings file and error if they are different"),
        )
        .arg(
            Arg::new("only-target-dependencies")
                .long("only-target-dependencies")
                .action(ArgAction::SetTrue)
                .help("Only fetch dependencies needed by the target platform. \
                    The target platform defaults to the host platform; set TARGET to override.")
        )
        .arg(
            Arg::new("clean")
                .long("clean")
                .action(ArgAction::SetTrue)
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
                .value_parser(value_parser!(PathBuf))
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
                .value_parser(value_parser!(PathBuf))
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
                .value_parser(value_parser!(PathBuf))
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
                .value_parser(value_parser!(PathBuf))
                .required(false),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .action(ArgAction::SetTrue)
                .help("Report errors only (overrides verbosity options).")
                .required(false),
        )
        .get_matches();

    // Initialize logging
    if matches.get_flag("quiet") {
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
    match matches.get_one::<PathBuf>("out") {
        Some(file) => {
            let changed = bindings.write_to_file(file);

            if matches.get_flag("verify") && changed {
                error!("Bindings changed: {}", file.display());
                std::process::exit(2);
            }
            if let Some(depfile) = matches.get_one("depfile") {
                bindings.generate_depfile(file, depfile);
            }
            if let Some(symfile) = matches.get_one::<String>("symfile") {
                bindings.generate_symfile(symfile);
            }
        }
        _ => {
            bindings.write(io::stdout());
        }
    }
}
