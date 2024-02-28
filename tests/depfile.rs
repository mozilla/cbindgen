use std::fs::read_to_string;
use std::path::PathBuf;
use std::process::Command;

static CBINDGEN_PATH: &str = env!("CARGO_BIN_EXE_cbindgen");

fn test_project(project_path: &str) {
    let mut cmake_cmd = Command::new("cmake");
    cmake_cmd.arg("--version");
    cmake_cmd
        .output()
        .expect("CMake --version failed - Is CMake installed?");

    let mut cmake_configure = Command::new("cmake");
    let build_dir = PathBuf::from(project_path).join("build");
    if build_dir.exists() {
        std::fs::remove_dir_all(&build_dir).expect("Failed to remove old build directory");
    }
    let project_dir = PathBuf::from(project_path);

    let cbindgen_define = format!("-DCBINDGEN_PATH={}", CBINDGEN_PATH);
    cmake_configure
        .arg("-S")
        .arg(project_path)
        .arg("-B")
        .arg(&build_dir)
        .arg(cbindgen_define);
    let output = cmake_configure.output().expect("Failed to execute process");
    let stdout_str = String::from_utf8(output.stdout).unwrap();
    let stderr_str = String::from_utf8(output.stderr).unwrap();
    assert!(
        output.status.success(),
        "Configuring test project failed: stdout: `{}`, stderr: `{}`",
        stdout_str,
        stderr_str
    );
    let depfile_path = build_dir.join("depfile.d");
    assert!(
        !depfile_path.exists(),
        "depfile should not exist before building"
    );

    // Do the clean first build
    let mut cmake_build = Command::new("cmake");
    cmake_build.arg("--build").arg(&build_dir);
    let output = cmake_build.output().expect("Failed to execute process");
    assert!(
        output.status.success(),
        "Building test project failed: {:?}",
        output
    );
    let out_str = String::from_utf8(output.stdout).unwrap();
    assert!(
        out_str.contains("Running cbindgen"),
        "cbindgen rule did not run. Output: {}",
        out_str
    );

    assert!(
        depfile_path.exists(),
        "depfile does not exist after building"
    );

    let expected_dependencies_filepath = PathBuf::from(project_path)
        .join("expectations")
        .join("dependencies");
    assert!(
        expected_dependencies_filepath.exists(),
        "Test did not define expected dependencies. Please read the Readme.md"
    );
    let expected_deps =
        read_to_string(expected_dependencies_filepath).expect("Failed to read dependencies");
    let depinfo = read_to_string(depfile_path).expect("Failed to read dependencies");
    // Assumes a single rule in the file - all deps are listed to the rhs of the `:`.
    let actual_deps = depinfo.split(':').collect::<Vec<_>>()[1];
    // Strip the line breaks.
    let actual_deps = actual_deps.replace("\\\n", " ");
    // I don't want to deal with supporting escaped whitespace when splitting at whitespace,
    // so the tests don't support being run in a directory containing whitespace.
    assert!(
        !actual_deps.contains("\\ "),
        "The tests directory may not contain any whitespace"
    );
    let dep_list: Vec<&str> = actual_deps.split_ascii_whitespace().collect();
    let expected_dep_list: Vec<String> = expected_deps
        .lines()
        .map(|dep| project_dir.join(dep).to_str().unwrap().to_string())
        .collect();
    assert_eq!(dep_list, expected_dep_list);

    let output = cmake_build.output().expect("Failed to execute process");
    assert!(
        output.status.success(),
        "Building test project failed: {:?}",
        output
    );
    let out_str = String::from_utf8(output.stdout).unwrap();
    assert!(
        !out_str.contains("Running cbindgen"),
        "cbindgen rule ran on second build"
    );

    std::fs::remove_dir_all(build_dir).expect("Failed to remove old build directory");
}

macro_rules! test_file {
    ($test_function_name:ident, $name:expr, $file:tt) => {
        #[test]
        fn $test_function_name() {
            test_project($file);
        }
    };
}

// This file is generated by build.rs
include!(concat!(env!("OUT_DIR"), "/depfile_tests.rs"));
