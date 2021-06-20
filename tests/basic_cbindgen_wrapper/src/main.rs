use std::{env, process};

fn exit_with_usage(exe_name: impl AsRef<str>) -> ! {
    eprintln!("Usage: {} MANIFEST_PATH PKG_NAME", exe_name.as_ref());
    process::exit(1);
}

/// A cbindgen wrapper whose return status can be leveraged by tests. Of note is that it is built
/// with `panic = "abort"` with respect to https://github.com/alexcrichton/proc-macro2/pull/220.
pub fn main() {
    let opts = env::args().collect::<Vec<String>>();
    if opts.len() < 3 {
        exit_with_usage(&opts[0]);
    }
    let crate_dir = &opts[1];
    let pkg_name = &opts[2];

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_parse_expand(&vec![pkg_name])
        .generate()
        .expect("Unable to generate bindings");
}
