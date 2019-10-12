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

use clap::{App, Arg, ArgMatches};

mod bindgen;
mod logging;

use bindgen::{Bindings, Builder, Cargo, Config, Error, Language, Style};

fn apply_config_overrides<'a>(config: &mut Config, matches: &ArgMatches<'a>) {
    // We allow specifying a language to override the config default. This is
    // used by compile-tests.
    if let Some(lang) = matches.value_of("lang") {
        config.language = match lang {
            "C++" => Language::Cxx,
            "c++" => Language::Cxx,
            "C" => Language::C,
            "c" => Language::C,
            _ => {
                error!("Unknown language specified.");
                return;
            }
        };
    }

    if matches.is_present("cpp-compat") {
        config.cpp_compat = true;
    }

    if let Some(style) = matches.value_of("style") {
        config.style = match style {
            "Both" => Style::Both,
            "both" => Style::Both,
            "Tag" => Style::Tag,
            "tag" => Style::Tag,
            "Type" => Style::Type,
            "type" => Style::Type,
            _ => {
                error!("Unknown style specified.");
                return;
            }
        }
    }

    if matches.is_present("d") {
        config.parse.parse_deps = true;
    }
}

fn load_bindings<'a>(input: &Path, matches: &ArgMatches<'a>) -> Result<Bindings, Error> {
    // If a file is specified then we load it as a single source
    if !input.is_dir() {
        // Load any config specified or search in the input directory
        let mut config = match matches.value_of("config") {
            Some(c) => Config::from_file(c).unwrap(),
            None => Config::from_root_or_default(input),
        };

        apply_config_overrides(&mut config, &matches);

        return Builder::new()
            .with_config(config)
            .with_src(input)
            .generate();
    }

    // We have to load a whole crate, so we use cargo to gather metadata
    let lib = Cargo::load(
        input,
        matches.value_of("lockfile"),
        matches.value_of("crate"),
        true,
        matches.is_present("clean"),
    )?;

    // Load any config specified or search in the binding crate directory
    let mut config = match matches.value_of("config") {
        Some(c) => Config::from_file(c).unwrap(),
        None => {
            let binding_crate_dir = lib.find_crate_dir(&lib.binding_crate_ref());

            if let Some(binding_crate_dir) = binding_crate_dir {
                Config::from_root_or_default(&binding_crate_dir)
            } else {
                // This shouldn't happen
                Config::from_root_or_default(input)
            }
        }
    };

    apply_config_overrides(&mut config, &matches);

    Builder::new()
        .with_config(config)
        .with_cargo(lib)
        .generate()
}

fn main() {
    let matches = App::new("cbindgen")
        .version(bindgen::VERSION)
        .about("Generate C bindings for a Rust library")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Enable verbose logging"),
        )
        .arg(
            Arg::with_name("verify")
                .long("verify")
                .help("Generate bindings and compare it to the existing bindings file and error if they are different"),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("PATH")
                .help("Specify path to a `cbindgen.toml` config to use"),
        )
        .arg(
            Arg::with_name("lang")
                .short("l")
                .long("lang")
                .value_name("LANGUAGE")
                .help("Specify the language to output bindings in")
                .possible_values(&["c++", "C++", "c", "C"]),
        )
        .arg(
            Arg::with_name("cpp-compat")
                .long("cpp-compat")
                .help("Whether to add C++ compatibility to generated C bindings")
        )
        .arg(
            Arg::with_name("style")
                .short("s")
                .long("style")
                .value_name("STYLE")
                .help("Specify the declaration style to use for bindings")
                .possible_values(&["Both", "both", "Tag", "tag", "Type", "type"]),
        )
        .arg(
            Arg::with_name("d")
                .short("d")
                .long("parse-dependencies")
                .help("Whether to parse dependencies when generating bindings"),
        )
        .arg(
            Arg::with_name("clean")
                .long("clean")
                .help(
                    "Whether to use a new temporary directory for expanding macros. \
                    Affects performance, but might be required in certain build processes.")
                .required(false)
        )
        .arg(
            Arg::with_name("INPUT")
                .help(
                    "A crate directory or source file to generate bindings for. \
                    In general this is the folder where the Cargo.toml file of \
                    source Rust library resides.")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::with_name("crate")
                .long("crate")
                .value_name("CRATE_NAME")
                .help(
                    "If generating bindings for a crate, \
                     the specific crate to generate bindings for",
                )
                .required(false),
        )
        .arg(
            Arg::with_name("out")
                .short("o")
                .long("output")
                .value_name("PATH")
                .help("The file to output the bindings to")
                .required(false),
        )
        .arg(
            Arg::with_name("lockfile")
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
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Report errors only (overrides verbosity options).")
                .required(false),
        )
        .get_matches();

    if !matches.is_present("out") && matches.is_present("verify") {
        error!(
            "Cannot verify bindings against `stdout`, please specify a file to compare against."
        );
        std::process::exit(2);
    }

    // Initialize logging
    if matches.is_present("quiet") {
        logging::ErrorLogger::init().unwrap();
    } else {
        match matches.occurrences_of("v") {
            0 => logging::WarnLogger::init().unwrap(),
            1 => logging::InfoLogger::init().unwrap(),
            _ => logging::TraceLogger::init().unwrap(),
        }
    }

    // Find the input directory
    let input = match matches.value_of("INPUT") {
        Some(input) => PathBuf::from(input),
        None => env::current_dir().unwrap(),
    };

    let bindings = match load_bindings(&input, &matches) {
        Ok(bindings) => bindings,
        Err(msg) => {
            error!("{}", msg);
            error!("Couldn't generate bindings for {}.", input.display());
            std::process::exit(1);
        }
    };

    // Write the bindings file
    match matches.value_of("out") {
        Some(file) => {
            let changed = bindings.write_to_file(file);

            if matches.is_present("verify") && changed {
                std::process::exit(2);
            }
        }
        _ => {
            bindings.write(io::stdout());
        }
    }
}
