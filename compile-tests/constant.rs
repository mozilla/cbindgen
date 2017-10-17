const FOO: i32 = 10;
const BAR: &'static libc::c_char = "hello world";

#[repr(C)]
struct Foo {
    x: [i32; FOO],
}

#[no_mangle]
extern "C" fn root(x: Foo) { }
