extern crate cbindgen;

mod utils;

mod constant {
    use cbindgen::Language;
    use utils::test_a_crate::test_a_crate;

    #[test]
    fn test_c() {
        test_a_crate("constant", Language::C);
    }

    #[test]
    fn test_cxx() {
        test_a_crate("constant", Language::Cxx);
    }
}
