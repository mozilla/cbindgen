pub const FOO: i32 = 10;
pub const BAR: &'static str = "hello world";
pub const DELIMITER: char = ':';
pub const LEFTCURLY: char = '{';
pub const QUOTE: char = '\'';
pub const TAB: char = '\t';
pub const NEWLINE: char = '\n';
pub const HEART: char = '❤';
pub const EQUID: char = '𐂃';
pub const ZOM: f32 = 3.14;

pub(crate) const DONT_EXPORT_CRATE: i32 = 20;
const DONT_EXPORT_PRIV: i32 = 30;

pub const POS_ONE: i8 = 1;
pub const NEG_ONE: i8 = -1;

// Some doc for shifting //
pub const SHIFT: i64 = 3;
pub const XBOOL: i64 = 1;
pub const XFALSE: i64 = (0 << SHIFT) | XBOOL;
pub const XTRUE: i64 = 1 << (SHIFT | XBOOL);

#[repr(C)]
struct Foo {
    x: [i32; FOO],
}

#[no_mangle]
pub extern "C" fn root(x: Foo) { }
