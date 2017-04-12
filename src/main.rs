use std::io;
use std::collections::HashSet;

extern crate syn;
extern crate clap;

use clap::{Arg, App};

mod rust_lib;
mod bindgen;

fn main() {
    let matches = App::new("cbindgen")
                    .version("0.1.0")
                    .about("Generate C bindings for a Rust library")
                    .arg(Arg::with_name("INPUT")
                         .help("The crate or source file to generate bindings for")
                         .required(true)
                         .index(1))
                    .get_matches();

    let crate_or_src = matches.value_of("INPUT").unwrap();

    let glyph_instance = bindgen::Prebuilt::new(
                                    String::from("GlyphInstance"),
                                    String::from(
r###"struct WrGlyphInstance {
  uint32_t index;
  Point2D point;

  bool operator==(const WrGlyphInstance& aOther) const {
    return index == aOther.index &&
      point == aOther.point;
  }
};"###));

    let lib = bindgen::Library::load(crate_or_src,
                                     vec![glyph_instance],
                                     HashSet::new());
    let built = lib.build().unwrap();

    built.write(&mut io::stdout());
}
