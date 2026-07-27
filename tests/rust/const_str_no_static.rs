use std::ffi::CStr;

// With `allow_static_const` and `allow_constexpr` both off, string, byte-string,
// and array constants fall back to the `#define` form in C++ too.
pub const C_STRING: &CStr = c"winner";
pub const BYTES: &[u8; 3] = b"abc";
pub const ARRAY: [&CStr; 2] = [c"first", c"second"];

#[no_mangle]
pub extern "C" fn root() {}
