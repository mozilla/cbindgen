use std::fs;
use std::fs::File;
use std::io::Read;

use cbindgen::Language;

const TEST_CRATES: &str = "test_crates";

pub fn test_a_crate(crate_to_test: &str, language: Language) {
    let mut libdir = fs::canonicalize(".").unwrap();
    libdir.push(TEST_CRATES);
    libdir.push(crate_to_test);

    let mut new_file_contents = Vec::new();

    cbindgen::Builder::new()
        .with_crate(&libdir)
        .with_language(language.clone())
        .generate()
        .expect(&format!("Unable to generate bindings {}", crate_to_test))
        .write(&mut new_file_contents);

    let ext = match language {
        Language::C => "c",
        Language::Cxx => "cpp",
    };

    let mut old_file_contents = Vec::new();
    {
        let mut old_file = File::open(libdir.join(format!("{}.{}", crate_to_test, ext))).unwrap();
        old_file.read_to_end(&mut old_file_contents).unwrap();
    }

    assert!(old_file_contents == new_file_contents);
}
