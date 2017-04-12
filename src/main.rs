use std::env;
use std::collections::HashSet;

extern crate syn;

mod rust_lib;
mod bindgen;

fn main() {
    let p = env::args().nth(1).unwrap();

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

    let lib = bindgen::Library::load(p, vec![glyph_instance], HashSet::new());
    let built = lib.build().unwrap();

    print!("{}", built.generate());
}
