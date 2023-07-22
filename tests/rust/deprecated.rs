#[no_mangle]
#[deprecated]
pub extern "C" fn deprecated_without_note() {}

#[no_mangle]
#[deprecated(note = "This is a note")]
pub extern "C" fn deprecated_with_note() {}
