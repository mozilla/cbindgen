/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io;
use std::path::Path;

extern crate clap;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate syn;
extern crate toml;
extern crate petgraph;

use clap::{Arg, ArgMatches, App};

mod logging;
mod bindgen;

use bindgen::{Cargo, Config, Language, Library};

fn apply_config_overrides<'a>(config: &mut Config, matches: &ArgMatches<'a>) {
    // We allow specifying a language to override the config default. This is
    // used by compile-tests.
    if let Some(lang) = matches.value_of("lang") {
        config.language = match lang {
            "C++"=> Language::Cxx,
            "c++"=> Language::Cxx,
            "C"=> Language::C,
            "c"=> Language::C,
            _ => {
                error!("unknown language specified");
                return;
            }
        };
    }

    if matches.is_present("d") {
        config.parse.parse_deps = true;
    }
}

fn load_library<'a>(input: &str, matches: &ArgMatches<'a>) -> Result<Library, String> {
    let input = Path::new(input);

    // If a file is specified then we load it as a single source
    if !input.is_dir() {
        // Load any config specified or search in the input directory
        let mut config = match matches.value_of("config") {
            Some(c) => Config::from_file(c).unwrap(),
            None => Config::from_root_or_default(input),
        };

        apply_config_overrides(&mut config, &matches);

        return Library::load_src(input, &config);
    }

    // We have to load a whole crate, so we use cargo to gather metadata
    let lib = Cargo::load(input,
                          matches.value_of("crate"),
                          true)?;

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

    Library::load_crate(lib, &config)
}

fn main() {
    let matches = App::new("cbindgen")
                    .version(bindgen::VERSION)
                    .about("Generate C bindings for a Rust library")
                    .arg(Arg::with_name("v")
                         .short("v")
                         .multiple(true)
                         .help("Enable verbose logging"))
                    .arg(Arg::with_name("config")
                         .short("c")
                         .long("config")
                         .value_name("PATH")
                         .help("Specify path to a `cbindgen.toml` config to use"))
                    .arg(Arg::with_name("lang")
                         .short("l")
                         .long("lang")
                         .value_name("LANGUAGE")
                         .help("Specify the language to output bindings in")
                         .possible_values(&["c++", "C++", "c", "C"]))
                    .arg(Arg::with_name("d")
                         .short("d")
                         .long("parse-dependencies")
                         .help("Whether to parse dependencies when generating bindings"))
                    .arg(Arg::with_name("INPUT")
                         .help("A crate directory or source file to generate bindings for")
                         .required(true)
                         .index(1))
                    .arg(Arg::with_name("crate")
                         .long("crate")
                         .value_name("CRATE_NAME")
                         .help("If generating bindings for a crate, the specific crate to generate bindings for")
                         .required(false))
                    .arg(Arg::with_name("out")
                         .short("o")
                         .long("output")
                         .value_name("PATH")
                         .help("The file to output the bindings to")
                         .required(false))
                    .get_matches();

    // Initialize logging
    match matches.occurrences_of("v") {
        0 => logging::WarnLogger::init().unwrap(),
        1 => logging::InfoLogger::init().unwrap(),
        _ => logging::TraceLogger::init().unwrap(),
    }

    // Find the input directory
    let input = matches.value_of("INPUT").unwrap();

    let library = match load_library(input, &matches) {
        Ok(library) => library,
        Err(msg) => {
            error!("{}", msg);
            error!("couldn't generate bindings for {}", input);
            return;
        }
    };

    // Generate a bindings file
    let built = match library.generate() {
        Ok(x) => x,
        Err(msg) => {
            error!("{}", msg);
            error!("couldn't generate bindings for {}", input);
            return;
        },
    };

    // Write the bindings file
    match matches.value_of("out") {
        Some(file) => {
            built.write_to_file(file);
        }
        _ => {
            built.write(io::stdout());
        }
    }
}
