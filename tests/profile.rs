use cbindgen::*;

use serial_test::serial;
use std::path::{Path, PathBuf};
use std::process::Command;

fn build_using_lib(config: fn(Builder) -> Builder) -> tempfile::TempDir {
    let expand_dep_test_dir = {
        let mut this_file = PathBuf::from(file!());
        this_file.pop();
        this_file.extend(&["rust", "expand_dep"]);
        this_file
    };

    let tmp_dir = tempfile::Builder::new()
        .prefix("cbindgen-test-output-")
        .tempdir()
        .expect("Creating tmp dir failed");

    std::env::set_var("CARGO_EXPAND_TARGET_DIR", tmp_dir.path());
    let builder = Builder::new()
        .with_config(Config::from_file(expand_dep_test_dir.join("cbindgen.toml")).unwrap())
        .with_crate(expand_dep_test_dir);
    let builder = config(builder);
    builder.generate().expect("build should succeed");

    tmp_dir
}

fn build_using_bin(extra_args: &[&str]) -> tempfile::TempDir {
    let expand_dep_test_dir = {
        let mut this_file = PathBuf::from(file!());
        this_file.pop();
        this_file.extend(&["rust", "expand_dep"]);
        this_file
    };

    let tmp_dir = tempfile::Builder::new()
        .prefix("cbindgen-test-output-")
        .tempdir()
        .expect("Creating tmp dir failed");

    let cbindgen_path = env!("CARGO_BIN_EXE_cbindgen");

    Command::new(cbindgen_path)
        .current_dir(expand_dep_test_dir)
        .env("CARGO_EXPAND_TARGET_DIR", tmp_dir.path())
        .args(extra_args)
        .output()
        .expect("build should succeed");

    tmp_dir
}

fn get_contents_of_dir(path: &Path) -> Vec<String> {
    path.read_dir()
        .unwrap()
        .map(|f| f.unwrap().file_name().to_str().unwrap().to_string())
        .filter(|name| !name.starts_with('.'))
        .collect()
}

#[test]
#[serial]
fn lib_default_uses_debug_build() {
    let target_dir = build_using_lib(|b| b);
    assert_eq!(get_contents_of_dir(target_dir.path()), &["debug"]);
}

#[test]
#[serial]
fn lib_explicit_debug_build() {
    let target_dir = build_using_lib(|b| b.with_parse_expand_profile(Profile::Debug));
    assert_eq!(get_contents_of_dir(target_dir.path()), &["debug"]);
}

#[test]
#[serial]
fn lib_explicit_release_build() {
    let target_dir = build_using_lib(|b| b.with_parse_expand_profile(Profile::Release));
    assert_eq!(get_contents_of_dir(target_dir.path()), &["release"]);
}

#[test]
fn bin_default_uses_debug_build() {
    let target_dir = build_using_bin(&[]);
    assert_eq!(get_contents_of_dir(target_dir.path()), &["debug"]);
}

#[test]
fn bin_explicit_debug_build() {
    let target_dir = build_using_bin(&["--profile", "debug"]);
    assert_eq!(get_contents_of_dir(target_dir.path()), &["debug"]);
}

#[test]
fn bin_explicit_release_build() {
    let target_dir = build_using_bin(&["--profile", "release"]);
    assert_eq!(get_contents_of_dir(target_dir.path()), &["release"]);
}
