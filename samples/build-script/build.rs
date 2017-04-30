extern crate cbindgen;

use std::env;
use cbindgen::{Config, Library};

fn main() {
    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let config = Config::from_root_or_default(&root);

    Library::load(&root, &config)
        .generate().unwrap()
        .write_to_file("bindings.h");
}
