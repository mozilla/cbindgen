#[no_mangle]
pub static NUMBER: i32 = 10;
#[no_mangle]
pub static STRING: &'static libc::c_char = "hello world";

#[repr(C)]
struct Foo {
}

struct Bar {
}

#[no_mangle]
pub static mut FOO: Foo = Foo { };
#[no_mangle]
pub static BAR: Bar = Bar { };

#[no_mangle]
pub extern "C" fn root() { }
