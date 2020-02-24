extern crate cbindgen;

use cbindgen::*;
use std::path::Path;
use std::process::Command;
use std::{env, fs, str};

fn style_str(style: Style) -> &'static str {
    match style {
        Style::Both => "both",
        Style::Tag => "tag",
        Style::Type => "type",
    }
}

fn run_cbindgen(
    cbindgen_path: &'static str,
    path: &Path,
    output: &Path,
    language: Language,
    cpp_compat: bool,
    style: Option<Style>,
) {
    let program = Path::new(cbindgen_path);
    let mut command = Command::new(&program);
    match language {
        Language::Cxx => {}
        Language::C => {
            command.arg("--lang").arg("c");

            if cpp_compat {
                command.arg("--cpp-compat");
            }
        }
    }

    if let Some(style) = style {
        command.arg("--style").arg(style_str(style));
    }

    command.arg("-o").arg(output);

    if env::var("CBINDGEN_TEST_VERIFY").is_ok() {
        command.arg("--verify");
    }

    let mut config = path.clone().to_path_buf();
    config.set_extension("toml");
    if config.exists() {
        command.arg("--config").arg(config);
    }

    command.arg(path);

    println!("Running: {:?}", command);
    let cbindgen_output = command.output().expect("failed to execute process");
    assert!(
        cbindgen_output.status.success(),
        "cbindgen failed: {:?} with error: {}",
        output,
        str::from_utf8(&cbindgen_output.stderr).unwrap_or_default()
    );
}

fn compile(cbindgen_output: &Path, tests_path: &Path, language: Language, style: Option<Style>) {
    let cc = match language {
        Language::Cxx => env::var("CXX").unwrap_or_else(|_| "g++".to_owned()),
        Language::C => env::var("CC").unwrap_or_else(|_| "gcc".to_owned()),
    };

    let mut object = cbindgen_output.to_path_buf();
    object.set_extension("o");

    let mut command = Command::new(cc);
    command.arg("-D").arg("DEFINED");
    command.arg("-c").arg(cbindgen_output);
    command.arg("-o").arg(&object);
    command.arg("-I").arg(tests_path);
    command.arg("-Wall");
    command.arg("-Werror");
    // `swift_name` is not recognzied by gcc.
    command.arg("-Wno-attributes");
    if let Language::Cxx = language {
        // enum class is a c++11 extension which makes g++ on macos 10.14 error out
        // inline variables are are a c++17 extension
        command.arg("-std=c++17");
        if let Ok(extra_flags) = env::var("CXXFLAGS") {
            command.args(extra_flags.split_whitespace());
        }
    } else {
        if let Ok(extra_flags) = env::var("CFLAGS") {
            command.args(extra_flags.split_whitespace());
        }
    }

    if let Some(style) = style {
        command.arg("-D");
        command.arg(format!(
            "CBINDGEN_STYLE_{}",
            style_str(style).to_uppercase()
        ));
    }

    println!("Running: {:?}", command);
    let out = command.output().expect("failed to compile");
    assert!(out.status.success(), "Output failed to compile: {:?}", out);

    if object.exists() {
        fs::remove_file(object).unwrap();
    }
}

fn run_compile_test(
    cbindgen_path: &'static str,
    name: &'static str,
    path: &Path,
    language: Language,
    cpp_compat: bool,
    style: Option<Style>,
) {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let tests_path = Path::new(&crate_dir).join("tests");
    let mut output = tests_path.join("expectations");
    if let Some(style) = style {
        match style {
            Style::Both => {
                output.push("both");
            }
            Style::Tag => {
                output.push("tag");
            }
            Style::Type => {}
        }
    }

    let ext = match language {
        Language::Cxx => "cpp",
        Language::C => {
            if cpp_compat {
                "compat.c"
            } else {
                "c"
            }
        }
    };

    output.push(format!("{}.{}", name, ext));

    run_cbindgen(cbindgen_path, path, &output, language, cpp_compat, style);
    compile(&output, &tests_path, language, style);

    if language == Language::C && cpp_compat {
        compile(&output, &tests_path, Language::Cxx, style)
    }
}

fn test_file(cbindgen_path: &'static str, name: &'static str, filename: &'static str) {
    let test = Path::new(filename);
    for style in &[Style::Type, Style::Tag, Style::Both] {
        for cpp_compat in &[true, false] {
            run_compile_test(
                cbindgen_path,
                name,
                &test,
                Language::C,
                *cpp_compat,
                Some(*style),
            );
        }
    }
    run_compile_test(
        cbindgen_path,
        name,
        &test,
        Language::Cxx,
        /* cpp_compat = */ false,
        None,
    );
}

macro_rules! test_file {
    ($cbindgen_path:expr, $test_function_name:ident, $name:expr, $file:tt) => {
        #[test]
        fn $test_function_name() {
            test_file($cbindgen_path, $name, $file);
        }
    };
}

// This file is generated by build.rs
include!(concat!(env!("OUT_DIR"), "/tests.rs"));
