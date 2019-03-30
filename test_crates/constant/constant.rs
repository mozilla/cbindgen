const FOO: usize = 10;
const BAR: &'static str = "hello world";
const ZOM: f32 = 3.14;

#[repr(C)]
pub struct Foo {
    x: [i32; FOO],
}

#[no_mangle]
pub extern "C" fn root(x: Foo) {}
