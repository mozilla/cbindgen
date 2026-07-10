use std::ffi::CStr;

pub const STRING: &str = "string";
pub const C_STRING: &CStr = c"c string";
pub const EMPTY: &str = "";

pub const WITH_QUOTES: &str = "a \"quoted\" word";
pub const WITH_BACKSLASH: &str = "back\\slash";
pub const WITH_CONTROLS: &str = "tab\tnewline\nreturn\r";
// Control bytes without a named escape, plus DEL (0x7f), become `\xNN`.
pub const OTHER_CONTROLS: &str = "\x01\x0b\x0c\x7f";

// `?` is escaped so no `??x` trigraph can form.
pub const TRIGRAPHS: &str = "what??!";

// A `&str` may contain an interior NUL; it survives as `\x00` (and forces a split
// before the following hex-digit character).
pub const INTERIOR_NUL: &str = "a\0b";

// A `&CStr` may hold non-UTF-8 bytes; they are escaped byte-for-byte.
pub const NON_UTF8: &CStr = c"\xff\xfe";

// Non-ASCII is emitted as the raw UTF-8 bytes escaped as `\xNN`, never Rust's
// `\u{...}`, which is not valid C.
pub const NON_ASCII: &str = "caf\u{e9}";

// A `\xNN` escape followed by a literal hex digit must not be absorbed into the
// escape: these stay two bytes, not one. Covers lower- and upper-case digits.
pub const HEX_ADJACENT: &CStr = c"\xc3a";
pub const HEX_ADJACENT_UPPER: &CStr = c"\xc3F";
// A `\xNN` escape followed by a non-hex character needs no split.
pub const HEX_NON_ADJACENT: &CStr = c"\xc3z";

pub const ARRAY: [&CStr; 2] = [c"first", c"second"];

/// A documented string constant.
pub const DOCUMENTED: &str = "documented";

#[no_mangle]
pub extern "C" fn root() {}
