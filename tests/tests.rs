extern crate cbindgen;

use cbindgen::*;
use std::io::Write;
use std::path::{Path, PathBuf};
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

fn prepare_compilation<'a>(
    name: &'static str,
    output_path: &'a Path,
    tmp_dir: &'a Path,
    tests_path: &'a Path,
    ext: &'a str
) -> (PathBuf, PathBuf, &'a Path) {
    
    let header_suffix = ".header";
    let header_position = name.rfind(header_suffix);

    if header_position.is_none() {
        let include_path = tests_path;
        let generate_file = output_path.join(format!("{}.{}", name, &ext));
        let compile_file = generate_file.clone();

        return (generate_file, compile_file, include_path);
    }

    let source_file = format!("{}.{}", &name[0..header_position.unwrap()], &ext);
    let mut generate_file = output_path.join(&source_file);
    let header_ext = match generate_file.extension().unwrap().to_str() {
        Some("cpp") => "hpp",
        Some("c") => "h",
        _ => "h",
    };
    generate_file.set_extension(header_ext);
    let temp_file_path = tmp_dir.join(source_file);
    let mut temp_source_file = fs::File::create(temp_file_path.as_path()).expect("unable to create temp source file");
    let source_file_content = format!(
        "#include \"{}\"\n",
        generate_file.file_name().unwrap().to_str().unwrap()
    );
    temp_source_file
        .write(source_file_content.as_bytes())
        .expect("unable to write source file content");

    return (generate_file, temp_file_path, output_path);
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

    let (generate_file, compile_file, include_path) = prepare_compilation(name, output.as_path(), tmp_dir, &tests_path, ext);

    run_cbindgen(cbindgen_path, path, &generate_file, language, cpp_compat, style);

    compile(&compile_file, &include_path, tmp_dir, language, style);

    if language == Language::C && cpp_compat {
        compile(&compile_file, &include_path, tmp_dir, Language::Cxx, style)
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
