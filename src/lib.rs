#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate syn;

mod bindgen;

pub use bindgen::*;
