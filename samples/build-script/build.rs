extern crate cbindgen;

use std::env;
use cbindgen::{Config, Library};

fn main() {
    let config = Config::from_file("cbindgen.toml");
    let source = env::var("CARGO_MANIFEST_DIR").unwrap();

    Library::load(&source, &config)
        .generate().unwrap()
        .write_to_file("bindings.h");
}
