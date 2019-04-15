const FOO: i32 = 10;
const BAR: &'static str = "hello world";
pub const DELIMITER: char = ':';
pub const QUOTE: char = '\'';
pub const TAB: char = '\t';
pub const NEWLINE: char = '\n';
pub const HEART: char = '❤';
const ZOM: f32 = 3.14;

#[repr(C)]
struct Foo {
    x: [i32; FOO],
}

#[no_mangle]
pub extern "C" fn root(x: Foo) { }
