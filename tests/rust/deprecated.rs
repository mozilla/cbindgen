#[no_mangle]
#[deprecated]
pub extern "C" fn deprecated_without_note() {}

#[no_mangle]
#[deprecated = "This is a note"]
pub extern "C" fn deprecated_without_bracket() {}

#[no_mangle]
#[deprecated(note = "This is a note")]
pub extern "C" fn deprecated_with_note() {}

#[no_mangle]
#[deprecated(note = "This is a note", since = "1.0.0")]
pub extern "C" fn deprecated_with_note_and_since() {}

#[no_mangle]
#[deprecated(note = "This quote \" requires to be quoted, and this [\n] requires to be escaped")]
pub extern "C" fn deprecated_with_note_which_requires_to_be_escaped() {}

#[derive(Debug, PartialEq, Eq)]
#[repr(i32)]
#[deprecated]
pub enum DeprecatedEnum {
    A = 0,
}

#[no_mangle]
pub extern "C" fn dummy(a: DeprecatedEnum) {}
