static NUMBER: i32 = 10;
static STRING: &'static libc::c_char = "hello world";

#[repr(C)]
struct Foo {
}

struct Bar {
}

static mut FOO: Foo = Foo { };
static BAR: Bar = Bar { };

#[no_mangle]
extern "C" fn root() { }
