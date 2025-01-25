//! This file tests assumptions that the clike language backend can make.
//!
//! FIXME ideally compile tests with `#[should_panic]` should give a warning result instead of a failure result

use cbindgen::Language;
use std::env;
use std::fs;
use std::process::Command;

fn compile(language: Language, code: &str) {
    // respect the desire to not compile (from tests.rs)
    // FIXME ideally it should skip the test
    let no_compile = env::var_os("CBINDGEN_TEST_NO_COMPILE").is_some();
    if no_compile {
        return;
    }

    let (compiler, filename) = match language {
        Language::C => (
            env::var("CC").unwrap_or_else(|_| "gcc".to_owned()),
            "test.c",
        ),
        Language::Cxx => (
            env::var("CXX").unwrap_or_else(|_| "g++".to_owned()),
            "test.cpp",
        ),
        _ => unreachable!(),
    };

    let tmp_dir = tempfile::Builder::new()
        .prefix("cbindgen-test-clike")
        .tempdir()
        .expect("Creating tmp dir failed");
    let tmp_dir = tmp_dir.path();

    let mut command = Command::new(compiler);
    command.arg("-Wall");
    command.arg("-Werror");
    command.arg("-c");
    command.arg(filename);
    command.current_dir(tmp_dir);

    fs::write(tmp_dir.join(filename), code).expect("Failed to write test code.");

    println!("Running: {:?}", command);
    let out = command.output().expect("failed to compile");

    // propagate command output to the test output for inspection
    println!("{}", String::from_utf8_lossy(&out.stdout));
    eprintln!("{}", String::from_utf8_lossy(&out.stderr));

    assert!(out.status.success(), "Output failed to compile: {:#?}", out);
}

/// TEST: the forward declaration "struct s;" is not needed?
mod auto_declared_struct {
    // ok in C, ok in Cxx
    const CODE: &'static str = r#"
    typedef struct s s_t;
    struct s {
        int f;
    };
    "#;

    use super::*;

    #[test]
    fn test_c() {
        compile(Language::C, CODE);
    }

    #[test]
    fn test_cxx() {
        compile(Language::Cxx, CODE);
    }
}

/// TEST: the forward declaration "union u;" is not needed?
mod auto_declared_union {
    // ok in C, ok in Cxx
    const CODE: &'static str = r#"
    typedef union u u_t;
    union u {
        int f;
    };
    "#;

    use super::*;

    #[test]
    fn test_c() {
        compile(Language::C, CODE);
    }

    #[test]
    fn test_cxx() {
        compile(Language::Cxx, CODE);
    }
}

/// TEST: the forward declaration "enum e;" is not needed?
mod auto_declared_enum {
    // ok in C, error in Cxx
    const CODE: &'static str = r#"
    typedef enum e e_t;
    enum e {
        V
    };
    "#;

    use super::*;

    #[test]
    fn test_c() {
        compile(Language::C, CODE);
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn test_cxx() {
        compile(Language::Cxx, CODE);
    }
}

/// TEST: "struct" does not need to be used?
mod struct_not_needed {
    // error in C, ok in Cxx
    const CODE: &'static str = r#"
    struct s {
        s* f;
    };
    "#;

    use super::*;

    #[test]
    #[ignore]
    #[should_panic]
    fn test_c() {
        compile(Language::C, CODE);
    }

    #[test]
    fn test_cxx() {
        compile(Language::Cxx, CODE);
    }
}

/// TEST: "union" does not need to be used?
mod union_not_needed {
    // error in C, ok in Cxx
    const CODE: &'static str = r#"
    union u {
        u* f;
    };
    "#;

    use super::*;

    #[test]
    #[ignore]
    #[should_panic]
    fn test_c() {
        compile(Language::C, CODE);
    }

    #[test]
    fn test_cxx() {
        compile(Language::Cxx, CODE);
    }
}

/// TEST: avoid warning `‘struct s’ declared inside parameter list will not be visible outside of this definition or declaration`
mod func_arg_warning {
    // warning in C, ok in Cxx
    const CODE: &'static str = r#"
    typedef int (*f_t)(struct s*);
    struct s {
        int f;
    };
    "#;

    use super::*;

    #[test]
    #[ignore]
    #[should_panic]
    fn test_c() {
        compile(Language::C, CODE);
    }

    #[test]
    fn test_cxx() {
        compile(Language::Cxx, CODE);
    }
}
