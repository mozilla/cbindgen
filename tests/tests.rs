extern crate cbindgen;

use cbindgen::*;
use std::io::Write;
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

fn compile(
    cbindgen_output: &Path,
    tests_path: &Path,
    tmp_dir: &Path,
    language: Language,
    style: Option<Style>,
) {
    let cc = match language {
        Language::Cxx => env::var("CXX").unwrap_or_else(|_| "g++".to_owned()),
        Language::C => env::var("CC").unwrap_or_else(|_| "gcc".to_owned()),
    };

    let file_name = cbindgen_output
        .file_name()
        .expect("cbindgen output should be a file");
    let mut object = tmp_dir.join(file_name);
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
    tmp_dir: &Path,
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

    let header_suffix = ".header";
    let header_position = name.rfind(&header_suffix);
    let is_header = header_position.is_some();

    let source_file = format!(
        "{}.{}",
        if is_header {
            &name[0..header_position.unwrap()]
        } else {
            name
        },
        &ext
    );
    output.push(&source_file);

    if is_header {
        let header_ext = match output.extension().unwrap().to_str().unwrap().as_ref() {
            "cpp" => "hpp",
            "c" => "h",
            _ => "",
        };
        let mut ofile = fs::File::create(output.as_path()).expect("unable to create surce file");
        output.set_extension(header_ext);
        let source_file_content = format!(
            "#include \"{}\"\n",
            output.file_name().unwrap().to_str().unwrap()
        );
        ofile
            .write(source_file_content.as_bytes())
            .expect("unable to write source file content");
    }

    run_cbindgen(cbindgen_path, path, &output, language, cpp_compat, style);

    if is_header {
        output.set_file_name(source_file);
    }

    compile(&output, &tests_path, tmp_dir, language, style);

    if language == Language::C && cpp_compat {
        compile(&output, &tests_path, tmp_dir, Language::Cxx, style)
    }
}

fn test_file(cbindgen_path: &'static str, name: &'static str, filename: &'static str) {
    let test = Path::new(filename);
    let tmp_dir = tempfile::Builder::new()
        .prefix("cbindgen-test-output")
        .tempdir()
        .expect("Creating tmp dir failed");
    let tmp_dir = tmp_dir.path();
    for style in &[Style::Type, Style::Tag, Style::Both] {
        for cpp_compat in &[true, false] {
            run_compile_test(
                cbindgen_path,
                name,
                &test,
                tmp_dir,
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
        tmp_dir,
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
