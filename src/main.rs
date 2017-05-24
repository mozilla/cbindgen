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

use clap::{Arg, App};

mod logging;
mod bindgen;

use bindgen::{Config, Language, Library};

fn main() {
    let matches = App::new("cbindgen")
                    .version(bindgen::VERSION)
                    .about("Generate C bindings for a Rust library")
                    .arg(Arg::with_name("v")
                         .short("v")
                         .help("whether to print verbose logs"))
                    .arg(Arg::with_name("config")
                         .short("c")
                         .long("config")
                         .value_name("CONFIG")
                         .help("the path to the cbindgen.toml config to use"))
                    .arg(Arg::with_name("lang")
                         .long("lang")
                         .value_name("LANGUAGE")
                         .help("the language to output bindings in: c++ or c, defaults to c++"))
                    .arg(Arg::with_name("INPUT")
                         .help("the crate or source file to generate bindings for")
                         .required(true)
                         .index(1))
                    .arg(Arg::with_name("crate")
                         .long("crate")
                         .value_name("CRATE")
                         .help("if generating bindings for a crate, the specific crate to create bindings for")
                         .required(false))
                    .arg(Arg::with_name("out")
                         .short("o")
                         .long("output")
                         .value_name("OUTPUT")
                         .help("the path to output the bindings to")
                         .required(false))
                    .get_matches();

    if matches.is_present("v") {
        logging::InfoLogger::init().unwrap();
    } else {
        logging::WarnLogger::init().unwrap();
    }

    let input = matches.value_of("INPUT").unwrap();

    let mut config = match matches.value_of("config") {
        Some(c) => Config::from_file(c).unwrap(),
        None => Config::from_root_or_default(&input),
    };

    if let Some(lang) = matches.value_of("lang") {
        config.language = match lang {
            "c++"=> Language::Cxx,
            "c"=> Language::C,
            _ => {
                error!("unknown language specified");
                return;
            }
        };
    }

    let library = if Path::new(&input).is_dir() {
        let binding_crate = matches.value_of("crate")
                                   .expect("--crate is required when building bindings for a library crate.");

        Library::load_crate(Path::new(input), &binding_crate, &config)
    } else {
        Library::load_src(Path::new(input), &config)
    };

    let built = match library.generate() {
        Ok(x) => x,
        Err(msg) => {
            error!("{}", msg);
            error!("could not generate bindings for {}", input);
            return;
        },
    };

    match matches.value_of("out") {
        Some(file) => {
            built.write_to_file(file);
        }
        _ => {
            built.write(io::stdout());
        }
    }
}
