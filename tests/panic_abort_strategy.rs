use std::{env, path::PathBuf, process::Command};

#[test]
fn test_panic_abort_strategy() {
    // Validates cbindgen's incorporation of https://github.com/alexcrichton/proc-macro2/issues/218.
    //
    // Run a binary whose profile specifies `panic = "abort"` and where the binary proceeds to
    // call into cbindgen's generation functionality. Prior to the incorporation of the above this
    // would result in the binary aborting.
    let cargo = env::var("CARGO").unwrap_or_else(|_| String::from("cargo"));
    let mut cmd = Command::new(cargo);

    let tests_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("tests");
    let wrapper_manifest_path = tests_dir.join("basic_cbindgen_wrapper").join("Cargo.toml");
    let arg_manifest_dir = tests_dir.join("rust").join("expand_dep");

    cmd.arg("run");
    cmd.arg("--manifest-path");
    cmd.arg(wrapper_manifest_path.as_path().to_str().unwrap());
    cmd.arg("--");
    cmd.arg(arg_manifest_dir.as_path().to_str().unwrap());
    cmd.arg("expand-dep");

    let output = cmd.output().expect("Failed to run cargo command");

    assert!(
        output.status.success(),
        "Cargo run failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
