extern crate cbindgen;

use cbindgen::{Config, Library};

fn main() {
    let config = Config::default();

    Library::load("../build-script", &config)
        .build(&config).unwrap()
        .write_to_file(&config, "bindings.h");
}
