use std::io;

extern crate clap;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate syn;
extern crate toml;

use clap::{Arg, App};

mod logging;
mod bindgen;

use bindgen::{Config, Library};

fn main() {
    let matches = App::new("cbindgen")
                    .version(bindgen::VERSION)
                    .about("Generate C bindings for a Rust library")
                    .arg(Arg::with_name("config")
                         .short("c")
                         .long("config")
                         .value_name("CONFIG")
                         .help("the config to use. currently either `wr`, or `default`"))
                    .arg(Arg::with_name("v")
                         .short("v")
                         .help("whether to print verbose logs"))
                    .arg(Arg::with_name("INPUT")
                         .help("the crate or source file to generate bindings for")
                         .required(true)
                         .index(1))
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

    let config = match matches.value_of("config") {
        Some(c) => Config::load(c),
        None => Config::from_root_or_default(&input),
    };

    let built = match Library::load(input, &config).generate() {
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
            built.write(&mut io::stdout());
        }
    }
}
