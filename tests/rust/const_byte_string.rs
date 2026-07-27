pub const BYTES: &[u8; 3] = b"abc";

// The unsized `&[u8]` slice form: the array length comes from the literal.
pub const SLICE: &[u8] = b"not null\0terminated";

// Non-printable and interior-NUL bytes survive because the value is a `uint8_t`
// array, not a C string literal.
pub const BINARY: &[u8; 4] = b"\x00\xff\x10\x7f";

// An empty byte string produces the same zero-length array as any other empty
// array constant (e.g. `[u8; 0]`); it is not special-cased here.
pub const EMPTY: &[u8] = b"";

/// A documented byte-string constant.
pub const DOCUMENTED: &[u8; 3] = b"doc";

#[no_mangle]
pub extern "C" fn root() {}
