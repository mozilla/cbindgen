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

#[no_mangle]
pub enum DeprecatedEnum {
    #[deprecated]
    DeprecatedVariant,
    #[deprecated = "This is a note"]
    DeprecatedVariantWithoutBracket,
    #[deprecated(note = "This is a note")]
    DeprecatedVariantWithNote,
    #[deprecated(note = "This is a note", since = "1.0.0")]
    DeprecatedVariantWithNoteAndSince,
}

#[no_mangle]
#[deprecated]
pub enum DeprecatedEnumItself {
    Item,
}
