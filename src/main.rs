use std::io;
use std::fs::File;
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
                         .help("the crate or source file to generate bindings for")
                         .required(true)
                         .index(1))
                    .arg(Arg::with_name("OUTPUT")
                         .help("the path to output the directories to")
                         .required(false)
                         .index(2))
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

    if let Some(out_file) = matches.value_of("OUTPUT") {
        built.write(&mut File::create(out_file).unwrap());
    } else {
        built.write(&mut io::stdout());
    }
}
