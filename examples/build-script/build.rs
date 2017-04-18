extern crate cbindgen;

use cbindgen::{Config, Library};

fn main() {
    let config = Config::default();

    Library::load("../build-script", &config)
        .generate().unwrap()
        .write_to_file("bindings.h");
}
